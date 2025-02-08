open Ast
open Core

let rec check ~loc (ctx : Context.t) (tm : Ast.term) (expected : Core.typ) : unit =
  match tm, expected with
  | WithLoc { value; loc }, expected -> check ~loc ctx value expected
  | Lambda (params, body), Func (params_ty, ret_ty) ->
    let ctx = Context.create (Some ctx) in
    List.iter2 (fun p ty -> Context.insert ctx p ty) params params_ty;
    check ~loc ctx body ret_ty
  | _, _ ->
    let actual = infer ~loc ctx tm in
    unify ~loc ~actual ~expected

and infer ~loc (ctx : Context.t) (tm : Ast.term) : Core.typ =
  match tm with
  | WithLoc { value; loc } -> infer ~loc ctx value
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
         let ty = infer ~loc ctx tm in
         Context.insert new_ctx name ty)
      bs;
    infer ~loc new_ctx t
  | Begin ts ->
    let t = List.hd @@ List.rev ts in
    infer ~loc ctx t
  | Lambda _ -> Reporter.fatalf Type_error "cannot infer lambda"
  | _ -> Reporter.fatalf TODO "TODO %s" ([%show: Ast.term] tm)

and unify ~loc ~(actual : Core.typ) ~(expected : Core.typ) : unit =
  match actual, expected with
  | Int, Int -> ()
  | Float, Float -> ()
  | Int, Float -> ()
  | _, _ ->
    Reporter.fatalf
      ~loc:(Option.get loc)
      Type_error
      "expected: %s, got: %s"
      ([%show: typ] expected)
      ([%show: typ] actual)
;;

let check_module (m : Expander.tapi_module) : unit =
  Hashtbl.iter
    (fun name tm ->
       let ty = Context.lookup m.context name in
       check ~loc:None m.context tm ty)
    m.tops
;;
