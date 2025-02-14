open Asai.Range

type term =
  | WithLoc of term located [@printer fun fmt t -> fprintf fmt "%s" (show_term t.value)]
  | Int of int [@printer fun fmt v -> fprintf fmt "%s" (string_of_int v)]
  | Rational of int * int
  [@printer fun fmt (p, q) -> fprintf fmt "%s/%s" (string_of_int p) (string_of_int q)]
  | Float of float [@printer fun fmt v -> fprintf fmt "%s" (string_of_float v)]
  | Bool of bool [@printer fun fmt v -> if v then fprintf fmt "#t" else fprintf fmt "#f"]
  | String of string [@printer fun fmt v -> fprintf fmt "%s" v]
  | Identifier of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | List of term list
  [@printer
    fun fmt xs -> fprintf fmt "(list %s)" (String.concat " " (List.map show_term xs))]
  | App of term * term list
  [@printer
    fun fmt (fn, args) ->
      fprintf fmt "(%s %s)" (show_term fn) (String.concat " " (List.map show_term args))]
  | If of term * term * term
  (* (if c t e) *)
  [@printer
    fun fmt (c, t, e) ->
      fprintf fmt "(if %s %s %s)" (show_term c) (show_term t) (show_term e)]
  | Let of binding list * term
  [@printer
    fun fmt (bs, b) ->
      fprintf
        fmt
        "(let (%s) %s)"
        (String.concat " " (List.map show_binding bs))
        (show_term b)]
  | Lambda of string list * term
  [@printer
    fun fmt (params, body) ->
      fprintf fmt "(lambda (%s) %s)" (String.concat " " params) (show_term body)]
  | Begin of term list
  [@printer
    fun fmt xs -> fprintf fmt "(begin %s)" (String.concat " " (List.map show_term xs))]

and binding =
  (string * term[@printer fun fmt (x, v) -> fprintf fmt "[%s %s]" x (show_term v)])
[@@deriving show]
