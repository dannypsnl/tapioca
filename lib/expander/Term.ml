type term =
  | Int of int
  | Rational of int * int
  | Float of float
  | Bool of bool
  | String of string
  | List of term list
