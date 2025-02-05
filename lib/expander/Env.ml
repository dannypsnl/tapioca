open Ast

type env =
  { bindings : (string, Ast.typ) Hashtbl.t
  ; parent : env option
  }

let create (parent : env option) : env = { bindings = Hashtbl.create 100; parent }

let insert (e : env) (id : string) (ty : Ast.typ) : unit =
  match Hashtbl.find_opt e.bindings id with
  | Some v ->
    Reporter.fatalf
      Type_error
      "`%s` be bound to second type `%s`, the original one is `%s`"
      id
      ([%show: typ] ty)
      ([%show: typ] v)
  | None -> Hashtbl.add e.bindings id ty
;;
