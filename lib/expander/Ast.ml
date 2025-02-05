open Asai.Range

type term =
  | WithLoc of term located
  | Int of int
  | Rational of int * int
  | Float of float
  | Bool of bool
  | String of string
  | List of term list

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
