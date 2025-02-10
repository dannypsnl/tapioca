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
    unify ~loc ~actual ~expected

and infer ~loc (ctx : Context.t) (tm : Ast.term) : Core.typ =
  match tm with
  | WithLoc { value; loc } -> infer ~loc:(Option.get loc) ctx value
  | Int _ -> Int
  | Rational _ -> Rational
  | Float _ -> Float
  | Bool _ -> Bool
  | String _ -> String
  | Identifier name -> Context.lookup ~loc ctx name
  | List tm_lst ->
    let typs = List.map (infer ~loc ctx) tm_lst in
    List (List.hd typs)
  | App (fn, args) ->
    let fn_ty = infer ~loc ctx fn in
    (match fn_ty with
     | Func (param_tys, ret_ty) ->
       check_args ~loc ctx args param_tys;
       ret_ty
     | _ -> Reporter.fatalf Type_error "%s cannot be applied" ([%show: Core.typ] fn_ty))
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

and check_args ~loc (ctx : Context.t) (tms : Ast.term list) (tys : Core.typ list) : unit =
  match tms, tys with
  (* (many t) will consume many t arguments, and from beginning there has no arguments should be also a fine case *)
  | [], [ Many _ ] -> ()
  (* (many t) should only be a valid type at the tail of parameter types *)
  | tm :: tms, [ Many ty ] ->
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

and unify ~loc ~(actual : Core.typ) ~(expected : Core.typ) : unit =
  match actual, expected with
  | Int, Int -> ()
  | Float, Float -> ()
  | Int, Number -> ()
  | Float, Number -> ()
  | String, String -> ()
  | _, Any -> ()
  | _, _ ->
    Reporter.fatalf
      ~loc
      Type_error
      "expected: %s, got: %s"
      ([%show: typ] expected)
      ([%show: typ] actual)
;;

let load_primitive_types (ctx : Context.t) : unit =
  Context.insert ctx "pretty-print" @@ Func ([ Any ], Void);
  (* TODO: a more proper type is numeric
      and if the inputs are int, the output can be int
      this is a far more complicated thing
  *)
  Context.insert ctx "+" @@ Func ([ Many Int ], Int);
  Context.insert ctx "-" @@ Func ([ Int; Many Int ], Int);
  Context.insert ctx "*" @@ Func ([ Many Int ], Int);
  Context.insert ctx "/" @@ Func ([ Int; Many Int ], Int);
  Context.insert ctx "decode-float" @@ Func ([ Float ], Vector Int);
  Context.insert ctx "string->number" @@ Func ([ String ], Number);
  Context.insert ctx "string-append-immutable" @@ Func ([ Many String ], Void);
  Context.insert ctx "string-truncate!" @@ Func ([ String; Int ], Void)
;;

let check_module (m : Expander.tapi_module) : unit =
  load_primitive_types m.context;
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
