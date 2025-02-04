open Asai.Range

type t =
  (notation located[@printer fun fmt t -> fprintf fmt "%s" (show_notation t.value)])

and notation =
  | Id of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | Bool of bool [@printer fun fmt v -> fprintf fmt "%s" (if v then "#t" else "#f")]
  | Int of int [@printer fun fmt i -> fprintf fmt "%s" (string_of_int i)]
  | Rational of int * int
  [@printer fun fmt (p, q) -> fprintf fmt "%s/%s" (string_of_int p) (string_of_int q)]
  | L of t list
  [@printer
    fun fmt xs ->
      fprintf
        fmt
        "(%s)"
        (String.concat " " (List.map (fun t -> show_notation t.value) xs))]
  | V of t list
  [@printer
    fun fmt xs ->
      fprintf
        fmt
        "#(%s)"
        (String.concat " " (List.map (fun t -> show_notation t.value) xs))]
[@@deriving show]
