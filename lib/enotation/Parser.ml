let loc_merge (loc : Asai.Range.t) (loc2 : Asai.Range.t option) : Asai.Range.t =
  match loc2 with
  | None -> loc
  | Some loc2 ->
    let s, _ = Asai.Range.split loc in
    let _, e = Asai.Range.split loc2 in
    Asai.Range.make (s, e)
;;

let rec enotation () : ENotation.t =
  let open Combinator in
  let open ENotation in
  let tok = next_token () in
  let loc = Option.get tok.loc in
  match tok.value with
  | IDENTIFIER s -> Asai.Range.locate loc @@ Id s
  | INTEGER i -> Asai.Range.locate loc @@ Int i
  | RATIONAL (p, q) -> Asai.Range.locate loc @@ Rational (p, q)
  | BOOL_TRUE -> Asai.Range.locate loc @@ Bool true
  | BOOL_FALSE -> Asai.Range.locate loc @@ Bool false
  | STRING s -> Asai.Range.locate loc @@ String s
  | OPEN_VECTOR ->
    let notations = Combinator.many enotation () in
    let loc2 = current_loc () in
    consume Lexer.CLOSE_PAREN;
    Asai.Range.locate (loc_merge loc loc2) @@ V notations
  | OPEN_PAREN ->
    let notations = Combinator.many enotation () in
    let loc2 = current_loc () in
    consume Lexer.CLOSE_PAREN;
    Asai.Range.locate (loc_merge loc loc2) @@ L notations
  | OPEN_BRACKET ->
    let notations = Combinator.many enotation () in
    let loc2 = current_loc () in
    consume Lexer.CLOSE_BRACKET;
    Asai.Range.locate (loc_merge loc loc2) @@ L notations
  | NOTATION_COMMENT ->
    (* a #; comment will ignore next enotation, and take the next next enotaion *)
    let _ = enotation () in
    enotation ()
  | CLOSE_PAREN -> raise CloseParen
  | CLOSE_BRACKET -> raise CloseParen
  | EOF -> raise EOF
;;

let notations () : ENotation.t list =
  let notations = Combinator.many enotation () in
  Combinator.consume Lexer.EOF;
  notations
;;

let rec tokens filename lexbuf =
  let tok = Lexer.token lexbuf in
  let loc =
    Asai.Range.of_lex_range
      ~source:(`File filename)
      (Sedlexing.lexing_bytes_positions lexbuf)
  in
  match tok with
  | EOF -> Asai.Range.locate loc tok :: []
  | tok -> Asai.Range.locate loc tok :: tokens filename lexbuf
;;

let parse_channel (filename : string) (ch : in_channel) : ENotation.t list =
  let lexbuf = Sedlexing.Utf8.from_channel ch in
  Sedlexing.set_filename lexbuf filename;
  Combinator.run (tokens filename lexbuf) notations
;;

let parse_file (filename : string) : ENotation.t list =
  let ch = open_in filename in
  Fun.protect ~finally:(fun _ -> close_in ch) @@ fun _ -> parse_channel filename ch
;;

(* This should not be invoke from outside, just for testing purpose *)
let test_parse_single (input : string) : ENotation.t =
  let lexbuf = Sedlexing.Utf8.from_string input in
  Sedlexing.set_filename lexbuf "test";
  Combinator.run (tokens "test" lexbuf) enotation
;;

let test_parse_many (input : string) : ENotation.t list =
  let lexbuf = Sedlexing.Utf8.from_string input in
  Sedlexing.set_filename lexbuf "test";
  Combinator.run (tokens "test" lexbuf) notations
;;

let%expect_test "identifier" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "x";
  [%expect {| x |}]
;;

let%expect_test "identifier weird" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "#%x";
  [%expect {| #%x |}]
;;

(* NOTE: `.` is not valid anymore, we preserve it for object accessing, therefore, the test case is a bit different from usual scheme *)
let%expect_test "identifier obsure" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "obscure-name-!$%^&*-_=+<>/?";
  [%expect {| obscure-name-!$%^&*-_=+<>/? |}]
;;

let%expect_test "identifier 漢字" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "世界";
  [%expect {| 世界 |}]
;;

let%expect_test "identifier 日文" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "本好きの下剋上";
  [%expect {| 本好きの下剋上 |}]
;;

let%expect_test "identifier quote" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "|6|";
  [%expect {| |6| |}]
;;

let%expect_test "identifier many" =
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "λ";
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "😇";
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "ok#";
  [%expect
    {|
    λ
    😇
    ok#
    |}]
;;

let%expect_test "boolean true" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "#t";
  [%expect {| #t |}]
;;

let%expect_test "boolean false" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "#f";
  [%expect {| #f |}]
;;

let%expect_test "integer" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "1";
  [%expect {| 1 |}]
;;

let%expect_test "rational" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "1/2";
  [%expect {| 1/2 |}]
;;

let%expect_test "a comment then an identifier" =
  print_string @@ [%show: ENotation.t] @@ test_parse_single "#;(x y z) x";
  [%expect {| x |}]
;;

let%expect_test "list" =
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "(x y z)";
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "(1 2 3)";
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "[1 2 3]";
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "([1 2 3] 4 5 6)";
  [%expect
    {|
    (x y z)
    (1 2 3)
    (1 2 3)
    ((1 2 3) 4 5 6)
    |}]
;;

let%expect_test "vector" =
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "#(1 2 3)";
  print_endline @@ [%show: ENotation.t] @@ test_parse_single "#(1 #(2 3))";
  [%expect
    {|
    #(1 2 3)
    #(1 #(2 3))
    |}]
;;
