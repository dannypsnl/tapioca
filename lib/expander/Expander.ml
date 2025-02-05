open Tapioca_enotation
open Tapioca_enotation.ENotation
open Ast
open Bwd
module Tty = Asai.Tty.Make (Reporter.Message)

exception SecondImport
exception BadImport
exception BadForm of ENotation.notation

type tapi_module =
  { mutable imports : string list option
  ; context : Context.context
  ; global_vars : (string, Ast.term) Hashtbl.t
  }

let create_module () : tapi_module =
  { imports = None; context = Context.create None; global_vars = Hashtbl.create 100 }
;;

let with_loc (f : ENotation.notation -> 'a) : ENotation.t -> 'a =
  fun n -> Reporter.with_loc n.loc @@ fun () -> f n.value
;;

let rec expand_file (ns : ENotation.t list) : tapi_module =
  let m = create_module () in
  List.iter (with_loc (expand_top (ref m))) ns;
  m

and expand_top (m : tapi_module ref) (n : ENotation.notation) =
  match n with
  | L ({ value = Id "import"; _ } :: modules) ->
    let modules = List.map (with_loc expand_id) modules in
    if Option.is_some !m.imports then raise SecondImport else !m.imports <- Some modules
  | L ({ value = Id ":"; _ } :: name :: { value = Id ":"; _ } :: ty) ->
    let name = (with_loc expand_id) name in
    let ty = expand_top_typ ty Emp in
    Context.insert !m.context name ty
  | L ({ value = Id "define"; _ } :: funcform :: bodys) ->
    (match funcform with
     | { value = Id name; _ } ->
       (match bodys with
        | [ { loc; value = body } ] ->
          let body : term = WithLoc { loc; value = expand_term body } in
          Hashtbl.add !m.global_vars name body
        | _ ->
          Reporter.fatalf
            Expander_error
            "expected only one expression here %s"
            ([%show: notation] n))
     | _ -> Reporter.fatalf TODO "TODO")
  | _ -> Reporter.fatalf Expander_error "bad form %s" ([%show: notation] n)

and expand_term : ENotation.notation -> term = function
  | Int i -> Int i
  | n -> Reporter.fatalf Expander_error "bad import form %s" ([%show: notation] n)

and expand_top_typ (ts : ENotation.t list) (stack : typ bwd) : typ =
  match ts with
  | { value = Id "->"; _ } :: ts ->
    let result_ty : typ = expand_top_typ ts Emp in
    Func (Bwd.to_list stack, result_ty)
  | { value; _ } :: ts -> expand_top_typ ts @@ Bwd.snoc stack (expand_typ value)
  | [] ->
    (match stack with
     | Snoc (Emp, v) -> v
     | _ -> Reporter.fatalf Expander_error "unable to expand out a type")

and expand_typ : ENotation.notation -> typ = function
  | Id "int" -> Int
  | n -> Reporter.fatalf Expander_error "bad import form %s" ([%show: notation] n)

and expand_id : ENotation.notation -> string = function
  | Id n -> n
  | n ->
    Reporter.fatalf
      Expander_error
      "expected an identifier, got `%s`"
      ([%show: notation] n)
;;

let test_runner (f : unit -> 'a) : 'a =
  let fatal diagnostics =
    Tty.display diagnostics;
    exit 1
  in
  Reporter.run ~emit:Tty.display ~fatal f
;;

let%expect_test "import" =
  test_runner
  @@ fun () ->
  let f = Tapioca_enotation.Parser.test_parse_many "(import rnrs)" in
  let m = expand_file f in
  (match m.imports with
   | Some s -> List.iter (fun i -> print_endline i) s
   | None -> print_endline "oops");
  [%expect {| rnrs |}]
;;

let%expect_test "import nothing" =
  test_runner
  @@ fun () ->
  let f = Tapioca_enotation.Parser.test_parse_many "(import )" in
  let m = expand_file f in
  (match m.imports with
   | Some s -> List.iter (fun i -> print_endline i) s
   | None -> print_endline "oops");
  [%expect {| |}]
;;
