open Tapioca_enotation.ENotation
module Tty = Asai.Tty.Make (Reporter.Message)

exception SecondImport
exception BadImport
exception BadForm of notation

type tapi_module = { mutable imports : string list option }

let rec expand_file (ns : notation list) : tapi_module =
  let m = { imports = None } in
  List.iter (expand_top (ref m)) ns;
  m

and expand_top (m : tapi_module ref) (n : notation) =
  match n with
  | WithLoc { loc; value } -> Reporter.with_loc loc @@ fun () -> expand_top m value
  | L (Id "import" :: modules) ->
    let modules = List.map expand_import modules in
    if Option.is_some !m.imports then raise SecondImport else !m.imports <- Some modules
  | _ -> Reporter.fatalf Expander_error "bad form %s" ([%show: notation] n)

and expand_import : notation -> string = function
  | WithLoc { loc; value } -> Reporter.with_loc loc @@ fun () -> expand_import value
  | Id n -> n
  | n -> Reporter.fatalf Expander_error "bad import form %s" ([%show: notation] n)
;;

let%expect_test "import" =
  let m = expand_file [ L [ Id "import"; Id "rnrs" ] ] in
  (match m.imports with
   | Some s -> List.iter (fun i -> print_endline i) s
   | None -> print_endline "oops");
  [%expect {| rnrs |}]
;;

let%expect_test "import nothing" =
  let m = expand_file [ L [ Id "import" ] ] in
  (match m.imports with
   | Some s -> List.iter (fun i -> print_endline i) s
   | None -> print_endline "oops");
  [%expect {| |}]
;;
