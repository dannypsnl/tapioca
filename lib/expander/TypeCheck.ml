open Ast
open Core

let rec check (ctx : Context.t) (tm : Ast.term) (expected : Core.typ) : unit =
  match tm, expected with
  | WithLoc { value; loc }, expected ->
    Reporter.trace ~loc:(Option.get loc) "term location"
    @@ fun () -> check ctx value expected
  | Lambda (params, body), Func (params_ty, ret_ty) ->
    let ctx = Context.create (Some ctx) in
    List.iter2 (fun p ty -> Context.insert ctx p ty) params params_ty;
    check ctx body ret_ty
  | _, _ ->
    let actual = infer ctx tm in
    unify ~actual ~expected

and infer (ctx : Context.t) (tm : Ast.term) : Core.typ =
  match tm with
  | WithLoc { value; _ } -> infer ctx value
  | Int _ -> Int
  | Rational _ -> Rational
  | Float _ -> Float
  | Bool _ -> Bool
  | String _ -> String
  | Identifier name -> Context.lookup ctx name
  | Let (bs, t) ->
    let new_ctx = Context.create (Some ctx) in
    List.iter
      (fun (name, tm) ->
         let ty = infer ctx tm in
         Context.insert new_ctx name ty)
      bs;
    infer new_ctx t
  | Begin ts ->
    let t = List.hd @@ List.rev ts in
    infer ctx t
  | Lambda _ -> Reporter.fatalf Type_error "cannot infer lambda"
  | _ -> Reporter.fatalf TODO "TODO %s" ([%show: Ast.term] tm)

and unify ~(actual : Core.typ) ~(expected : Core.typ) : unit =
  match actual, expected with
  | Int, Int -> ()
  | _, _ -> Reporter.fatalf TODO "TODO"
;;

let check_module (m : Expander.tapi_module) : unit =
  Hashtbl.iter
    (fun name tm ->
       let ty = Context.lookup m.context name in
       check m.context tm ty)
    m.tops
;;
