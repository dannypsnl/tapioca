open Asai.Range

type notation =
  | WithLoc of notation located
  [@printer fun fmt { loc = _; value } -> fprintf fmt "%s" (show_notation value)]
  | Id of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | L of notation list
  [@printer
    fun fmt xs -> fprintf fmt "(%s)" (String.concat " " (List.map show_notation xs))]
[@@deriving show]
