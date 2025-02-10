type typ =
  | Func of typ list * typ
  [@printer
    fun fmt (ts, t) ->
      fprintf fmt "%s -> %s" (String.concat " " (List.map show_typ ts)) (show_typ t)]
  | Void [@printer fun fmt _ -> fprintf fmt "void"]
  | Any [@printer fun fmt _ -> fprintf fmt "any"]
  | Number [@printer fun fmt _ -> fprintf fmt "number"]
  | Int [@printer fun fmt _ -> fprintf fmt "int"]
  | Rational [@printer fun fmt _ -> fprintf fmt "rational"]
  (* In some language, this is called double type *)
  | Float [@printer fun fmt _ -> fprintf fmt "float"]
  | Bool [@printer fun fmt _ -> fprintf fmt "bool"]
  | String [@printer fun fmt _ -> fprintf fmt "string"]
  | Many of typ [@printer fun fmt t -> fprintf fmt "(many %s)" (show_typ t)]
  | List of typ [@printer fun fmt t -> fprintf fmt "(list %s)" (show_typ t)]
  | Vector of typ [@printer fun fmt t -> fprintf fmt "(vector %s)" (show_typ t)]
[@@deriving show]
