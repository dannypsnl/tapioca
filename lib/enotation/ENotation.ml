open Asai.Range

type notation =
  | WithLoc of notation located
  [@printer fun fmt { loc = _; value } -> fprintf fmt "%s" (show_notation value)]
  | Id of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | Bool of bool [@printer fun fmt v -> fprintf fmt "%s" (if v then "#t" else "#f")]
  | Int of int [@printer fun fmt i -> fprintf fmt "%s" (string_of_int i)]
  | Rational of int * int
  [@printer fun fmt (p, q) -> fprintf fmt "%s/%s" (string_of_int p) (string_of_int q)]
  | L of notation list
  [@printer
    fun fmt xs -> fprintf fmt "(%s)" (String.concat " " (List.map show_notation xs))]
[@@deriving show]
