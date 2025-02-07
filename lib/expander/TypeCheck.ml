open Ast

let rec check (ctx : Context.t) (tm : Ast.term) (expected : Ast.typ) : unit =
  let actual = infer ctx tm in
  unify ~actual ~expected

and infer (ctx : Context.t) (tm : Ast.term) : Ast.typ =
  match tm with
  | Identifier name -> Context.lookup ctx name
  | _ -> Reporter.fatalf TODO "TODO"

and unify ~(actual : Ast.typ) ~(expected : Ast.typ) : unit =
  let _ = actual in
  let _ = expected in
  ()
;;

let check_module (m : Expander.tapi_module) : unit =
  Hashtbl.iter
    (fun name tm ->
       let ty = Context.lookup m.context name in
       check m.context tm ty)
    m.tops
;;
