open Core

type t =
  { bindings : (string, Core.typ) Hashtbl.t
  ; parent : t option
  }

let create (parent : t option) : t = { bindings = Hashtbl.create 100; parent }

let insert (ctx : t) (id : string) (ty : Core.typ) : unit =
  match Hashtbl.find_opt ctx.bindings id with
  | Some v ->
    Reporter.fatalf
      Type_error
      "`%s` be bound to second type `%s`, the original one is `%s`"
      id
      ([%show: typ] ty)
      ([%show: typ] v)
  | None -> Hashtbl.add ctx.bindings id ty
;;

let rec lookup ~loc (ctx : t) (id : string) : Core.typ =
  match Hashtbl.find_opt ctx.bindings id with
  | None -> begin
    match ctx.parent with
    | None -> Reporter.fatalf ~loc Type_error "`%s` has no type" id
    | Some p -> lookup ~loc p id
  end
  | Some v -> v
;;

let import (ctx : t) (another : t) : unit =
  Hashtbl.iter (fun name ty -> insert ctx name ty) another.bindings
;;
