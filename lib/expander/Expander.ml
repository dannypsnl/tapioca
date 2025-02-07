open Tapioca_enotation
open Tapioca_enotation.ENotation
open Ast
open Bwd
module Tty = Asai.Tty.Make (Reporter.Message)

exception SecondImport
exception BadImport
exception BadForm of ENotation.notation

type tapi_module =
  { filename : string
  ; mutable imports : string list option
  ; context : Context.context
  ; tops : (string, Ast.term) Hashtbl.t
  ; program : Ast.term Dynarray.t
  }

let create_module (filename : string) : tapi_module =
  { filename
  ; imports = None
  ; context = Context.create None
  ; tops = Hashtbl.create 100
  ; program = Dynarray.create ()
  }
;;

let with_loc (f : ENotation.notation -> 'a) : ENotation.t -> 'a =
  fun n -> Reporter.with_loc n.loc @@ fun () -> f n.value
;;

let rec expand_file (filename : string) (ns : ENotation.t list) : tapi_module =
  let m = create_module filename in
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
  | L ({ value = Id "define"; _ } :: { value = Id name; _ } :: [ { loc; value = body } ])
    ->
    let term : term = WithLoc { loc; value = expand_term body } in
    Hashtbl.add !m.tops name term
  | L ({ value = Id "define"; _ } :: { value = Id _; _ } :: _) ->
    Reporter.fatalf Expander_error "expected only one expression here"
  | L ({ value = Id "define"; _ } :: funcform :: bodys) ->
    let name, term = (with_loc (expand_func_form bodys)) funcform in
    Hashtbl.add !m.tops name term
  | _ ->
    let tm = expand_term n in
    Dynarray.add_last !m.program tm

and expand_func_form (bodys : ENotation.t list) : ENotation.notation -> string * term
  = function
  | Id name -> name, wrap_begin bodys
  | L (head :: params) ->
    let params = List.map (with_loc expand_id) params in
    let name, body = (with_loc (expand_func_form bodys)) head in
    name, Lambda (params, body)
  | _ -> Reporter.fatalf Expander_error "not a valid function form"

and expand_term : ENotation.notation -> term = function
  | Int i -> Int i
  | Rational (p, q) -> Rational (p, q)
  | Bool b -> Bool b
  | Id x -> Identifier x
  | L ({ value = Id "let"; _ } :: { value = L bindings; _ } :: bodys) ->
    let bindings = List.map (with_loc expand_binding) bindings in
    let body = wrap_begin bodys in
    Let (bindings, body)
  | L ({ value = Id "lambda"; _ } :: { value = L params; _ } :: bodys) ->
    let params = List.map (with_loc expand_id) params in
    let body = wrap_begin bodys in
    Lambda (params, body)
  | L (fn :: args) ->
    let fn = (with_loc expand_term) fn in
    let args : term list =
      List.map
        (fun (t : ENotation.t) ->
           (WithLoc { loc = t.loc; value = expand_term t.value } : term))
        args
    in
    App (fn, args)
  | n -> Reporter.fatalf Expander_error "bad form %s" ([%show: notation] n)

and expand_binding : ENotation.notation -> string * term = function
  | L [ { value = Id name; _ }; expr ] -> name, (with_loc expand_term) expr
  | _ -> Reporter.fatalf Expander_error "bad binding"

and wrap_begin (bodys : ENotation.t list) : term =
  let bodys : term list =
    List.map
      (fun (t : ENotation.t) ->
         (WithLoc { loc = t.loc; value = expand_term t.value } : term))
      bodys
  in
  Begin bodys

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
  | Id "bool" -> Bool
  | Id "string" -> String
  | Id "int" -> Int
  | Id "rational" -> Rational
  | Id "float" -> Float
  | L ({ value = Id "list"; _ } :: t) -> List (expand_top_typ t Emp)
  | L ({ value = Id "vector"; _ } :: t) -> Vector (expand_top_typ t Emp)
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
  let m = expand_file "test" f in
  (match m.imports with
   | Some s -> List.iter (fun i -> print_endline i) s
   | None -> print_endline "oops");
  [%expect {| rnrs |}]
;;

let%expect_test "import nothing" =
  test_runner
  @@ fun () ->
  let f = Tapioca_enotation.Parser.test_parse_many "(import )" in
  let m = expand_file "test" f in
  (match m.imports with
   | Some s -> List.iter (fun i -> print_endline i) s
   | None -> print_endline "oops");
  [%expect {| |}]
;;
