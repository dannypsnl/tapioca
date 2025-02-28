type typ =
  | Func of typ list * typ
  [@printer
    fun fmt (ts, t) ->
      fprintf fmt "%s -> %s" (String.concat " " (List.map show_typ ts)) (show_typ t)]
  | Or of typ list
  [@printer fun fmt ts -> fprintf fmt "(⊎ %s)" (String.concat " " (List.map show_typ ts))]
  | Values of typ list
  [@printer
    fun fmt ts -> fprintf fmt "(values %s)" (String.concat " " (List.map show_typ ts))]
  | Many of typ [@printer fun fmt t -> fprintf fmt "%s ..." (show_typ t)]
  | Optional of typ [@printer fun fmt t -> fprintf fmt "(? %s)" (show_typ t)]
  | List of typ [@printer fun fmt t -> fprintf fmt "(list %s)" (show_typ t)]
  | Vector of typ [@printer fun fmt t -> fprintf fmt "(vector %s)" (show_typ t)]
  | Void [@printer fun fmt _ -> fprintf fmt "void"]
  | Any [@printer fun fmt _ -> fprintf fmt "any"]
  | Number [@printer fun fmt _ -> fprintf fmt "number"]
  | U8 [@printer fun fmt _ -> fprintf fmt "u8"]
  | U16 [@printer fun fmt _ -> fprintf fmt "u16"]
  | Int [@printer fun fmt _ -> fprintf fmt "int"]
  | IntLit of int [@printer fun fmt v -> fprintf fmt "(int@%s)" (string_of_int v)]
  | Rational [@printer fun fmt _ -> fprintf fmt "rational"]
  (* In some language, this is called double type *)
  | Float [@printer fun fmt _ -> fprintf fmt "float"]
  | Bool [@printer fun fmt _ -> fprintf fmt "bool"]
  | String [@printer fun fmt _ -> fprintf fmt "string"]
  | OutputPort [@printer fun fmt _ -> fprintf fmt "output-port"]
[@@deriving show]
