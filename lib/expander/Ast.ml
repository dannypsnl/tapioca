open Asai.Range

type term =
  | WithLoc of term located [@printer fun fmt t -> fprintf fmt "%s" (show_term t.value)]
  | Int of int [@printer fun fmt v -> fprintf fmt "%s" (string_of_int v)]
  | Rational of int * int
  [@printer fun fmt (p, q) -> fprintf fmt "%s/%s" (string_of_int p) (string_of_int q)]
  | Float of float [@printer fun fmt v -> fprintf fmt "%s" (string_of_float v)]
  | Bool of bool [@printer fun fmt v -> if v then fprintf fmt "#t" else fprintf fmt "#f"]
  | String of string [@printer fun fmt v -> fprintf fmt "\"%s\"" v]
  | Identifier of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | List of term list
  [@printer
    fun fmt xs -> fprintf fmt "(list %s)" (String.concat " " (List.map show_term xs))]
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

type typ =
  | WithLoc of typ located [@printer fun fmt t -> fprintf fmt "%s" (show_typ t.value)]
  | Func of typ list * typ
  [@printer
    fun fmt (ts, t) ->
      fprintf fmt "%s -> %s" (String.concat " " (List.map show_typ ts)) (show_typ t)]
  | Int [@printer fun fmt _ -> fprintf fmt "int"]
  | Rational [@printer fun fmt _ -> fprintf fmt "rational"]
  (* In some language, this is called double type *)
  | Float [@printer fun fmt _ -> fprintf fmt "float"]
  | Bool [@printer fun fmt _ -> fprintf fmt "bool"]
  | String [@printer fun fmt _ -> fprintf fmt "string"]
  | List of typ [@printer fun fmt t -> fprintf fmt "(list %s)" (show_typ t)]
  | Vector of typ [@printer fun fmt t -> fprintf fmt "(vector %s)" (show_typ t)]
[@@deriving show]
