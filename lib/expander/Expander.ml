open Tapioca_enotation
open Tapioca_enotation.ENotation
module Tty = Asai.Tty.Make (Reporter.Message)

exception SecondImport
exception BadImport
exception BadForm of ENotation.notation

type tapi_module = { mutable imports : string list option }

let expand_t (f : ENotation.notation -> 'a) : ENotation.t -> 'a =
  fun n -> Reporter.with_loc n.loc @@ fun () -> f n.value
;;

let rec expand_file (ns : ENotation.t list) : tapi_module =
  let m = { imports = None } in
  List.iter (expand_t (expand_top (ref m))) ns;
  m

and expand_top (m : tapi_module ref) (n : ENotation.notation) =
  match n with
  | L ({ value = Id "import"; _ } :: modules) ->
    let modules = List.map (expand_t expand_id) modules in
    if Option.is_some !m.imports then raise SecondImport else !m.imports <- Some modules
  | L ({ value = Id ":"; _ } :: name :: { value = Id ":"; _ } :: ty) ->
    let _ = (expand_t expand_id) name in
    let _ = ty in
    Reporter.fatalf TODO "TODO %s" ([%show: notation] n)
  | L ({ value = Id "define"; _ } :: funcform :: bodys) ->
    (match funcform with
     | { value = Id name; _ } ->
       (match bodys with
        | [ body ] ->
          let _ = name in
          let _ = body in
          Reporter.fatalf TODO "TODO"
        | _ ->
          Reporter.fatalf
            Expander_error
            "expected only one body here %s"
            ([%show: notation] n))
     | _ -> Reporter.fatalf TODO "TODO")
  | _ -> Reporter.fatalf Expander_error "bad form %s" ([%show: notation] n)

and expand_expression : ENotation.notation -> Term.term = function
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
