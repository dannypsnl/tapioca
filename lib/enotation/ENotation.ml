open Asai.Range

type notation =
  | WithLoc of notation located
  [@printer fun fmt { loc = _; value = no } -> fprintf fmt "%s" (show_notation no)]
  | Id of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | L of notation list
  [@printer
    fun fmt xs -> fprintf fmt "()" (String.concat " " (List.map show_notation xs))]
[@@derving show]

(* let%expect_test "addition" =
  print_string @@ [%derive.show: notation] (Id "x");
  [%expect {| x |}]
;; *)
