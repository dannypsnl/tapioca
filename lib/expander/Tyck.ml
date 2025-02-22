open Ast
open Core
open Asai.Range

let rec check ~loc (ctx : Context.t) (tm : Ast.term) (expected : Core.typ) : unit =
  match tm, expected with
  | WithLoc { value; loc }, expected -> check ~loc:(Option.get loc) ctx value expected
  | Lambda (params, body), Func (params_ty, ret_ty) ->
    let ctx = Context.create (Some ctx) in
    List.iter2 (fun p ty -> Context.insert ctx p ty) params params_ty;
    check ~loc ctx body ret_ty
  | _, _ ->
    let actual = infer ~loc ctx tm in
    if unify actual expected
    then ()
    else
      Reporter.fatalf
        ~loc
        Type_error
        "expected: %s, got: %s"
        ([%show: typ] expected)
        ([%show: typ] actual)

and infer ~loc (ctx : Context.t) (tm : Ast.term) : Core.typ =
  match tm with
  | WithLoc { value; loc } -> infer ~loc:(Option.get loc) ctx value
  | Int v -> IntLit v
  | Rational _ -> Rational
  | Float _ -> Float
  | Bool _ -> Bool
  | String _ -> String
  | Identifier name -> Context.lookup ~loc ctx name
  | List tm_lst ->
    let typs = List.map (infer ~loc ctx) tm_lst in
    List (List.hd typs)
  | App (fn, args) -> begin
    let fn_ty = infer ~loc ctx fn in
    match fn_ty with
    | Func (param_tys, ret_ty) ->
      check_args ~loc ctx args param_tys;
      ret_ty
    | _ -> Reporter.fatalf Type_error "%s cannot be applied" ([%show: Core.typ] fn_ty)
  end
  | If (c, t, e) ->
    check ~loc ctx c Bool;
    let then_ty = infer ~loc ctx t in
    let else_ty = infer ~loc ctx e in
    if unify else_ty then_ty then then_ty else Or [ then_ty; else_ty ]
  | Let (bs, t) ->
    let new_ctx = Context.create (Some ctx) in
    List.iter
      (fun (name, tm) ->
         let ty = infer ~loc ctx tm in
         Context.insert new_ctx name ty)
      bs;
    infer ~loc new_ctx t
  | LetValues (bs, t) ->
    let new_ctx = Context.create (Some ctx) in
    List.iter
      (fun binding ->
         match binding with
         | FormalsBinding (names, tm) ->
           let ty = infer ~loc ctx tm in
           (match ty with
            | Values tys ->
              let _ =
                List.map2 (fun name ty -> Context.insert new_ctx name ty) names tys
              in
              ()
            | ty ->
              Reporter.fatalf
                Type_error
                "%s didn't match with formals"
                ([%show: Core.typ] ty))
         | Binding (name, tm) ->
           let ty = infer ~loc ctx tm in
           Context.insert new_ctx name ty)
      bs;
    infer ~loc new_ctx t
  | Begin ts ->
    let t = List.hd @@ List.rev ts in
    infer ~loc ctx t
  | Lambda _ -> Reporter.fatalf Type_error "cannot infer lambda"

and check_args ~loc (ctx : Context.t) (tms : Ast.term list) (tys : Core.typ list) : unit =
  match tms, tys with
  (* (many t) will consume many t arguments, and from beginning there has no arguments should be also a fine case *)
  | [], [ Many _ ] -> ()
  (* (many t) should only be a valid type at the tail of parameter types *)
  | tm :: tms, [ Many ty ] ->
    check ~loc ctx tm ty;
    check_args ~loc ctx tms tys
  | [], Optional _ :: _ -> ()
  | tm :: tms, Optional ty :: tys ->
    check ~loc ctx tm ty;
    check_args ~loc ctx tms tys
  | tm :: tms, ty :: tys ->
    check ~loc ctx tm ty;
    check_args ~loc ctx tms tys
  | [], [] -> ()
  | tms, [] ->
    Reporter.fatalf
      Type_error
      "expected no types, but still have arguments %s"
      (String.concat " " (List.map show_term tms))
  | [], ty :: _ ->
    Reporter.fatalf ~loc Type_error "expected: %s, but has no arguments" ([%show: typ] ty)

and unify (actual : Core.typ) (expected : Core.typ) : bool =
  match actual, expected with
  | Bool, Bool -> true
  | U8, Int | Int, Int | IntLit _, Int -> true
  | U8, U8 -> true
  | IntLit v, U8 -> v <= 255 && 0 <= v
  | Float, Float -> true
  | U8, Number | IntLit _, Number | Int, Number | Float, Number | Number, Number -> true
  | String, String -> true
  | OutputPort, OutputPort -> true
  | _, Any -> true
  | _, _ -> false
;;

let check_module (m : Expander.tapi_module) : unit =
  Hashtbl.iter
    (fun name { value = tm; loc } ->
       let loc = Option.get loc in
       let ty = Context.lookup ~loc m.context name in
       check ~loc m.context tm ty)
    m.tops;
  Dynarray.iter
    (fun { value = tm; loc } ->
       let loc = Option.get loc in
       check ~loc m.context tm Any)
    m.program
;;
