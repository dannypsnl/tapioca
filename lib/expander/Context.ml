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

let rec lookup (ctx : t) (id : string) : Core.typ =
  match ctx.parent with
  | None -> Hashtbl.find ctx.bindings id
  | Some p ->
    (match Hashtbl.find_opt ctx.bindings id with
     | None -> lookup p id
     | Some v -> v)
;;
