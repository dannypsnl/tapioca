exception
  UnexpectedToken of
    { loc : Asai.Range.t
    ; tok : Lexer.token
    }

let rec enotation () : ENotation.notation =
  let open Combinator in
  let open ENotation in
  let tok = next_token () in
  let loc = Option.get tok.loc in
  let notation =
    match tok.value with
    | IDENTIFIER s -> Id s
    | INTEGER i -> Int i
    | RATIONAL (p, q) -> Rational (p, q)
    | BOOL_TRUE -> Bool true
    | BOOL_FALSE -> Bool false
    | OPEN_VECTOR ->
      let notations = Combinator.many enotation () in
      consume Lexer.CLOSE_PAREN;
      V notations
    | OPEN_PAREN ->
      let notations = Combinator.many enotation () in
      consume Lexer.CLOSE_PAREN;
      L notations
    | OPEN_BRACKET ->
      let notations = Combinator.many enotation () in
      consume Lexer.CLOSE_BRACKET;
      L notations
    | NOTATION_COMMENT ->
      (* a #; comment will ignore next enotation, and take the next next enotaion *)
      let _ = enotation () in
      enotation ()
    | CLOSE_PAREN -> raise CloseParen
    | CLOSE_BRACKET -> raise CloseParen
    | tok -> raise (UnexpectedToken { loc; tok })
  in
  WithLoc (Asai.Range.locate loc notation)
;;

let notations () : ENotation.notation list =
  let notations = Combinator.many enotation () in
  Combinator.consume Lexer.EOF;
  notations
;;

let rec tokens filename lexbuf =
  let tok = Lexer.token lexbuf in
  let loc =
    Asai.Range.of_lex_range
      ~source:(`File filename)
      (Sedlexing.lexing_position_start lexbuf, Sedlexing.lexing_position_curr lexbuf)
  in
  match tok with
  | EOF -> Asai.Range.locate loc tok :: []
  | tok -> Asai.Range.locate loc tok :: tokens filename lexbuf
;;

let parse_channel (filename : string) (ch : in_channel) : ENotation.notation list =
  let lexbuf = Sedlexing.Utf8.from_channel ch in
  Sedlexing.set_filename lexbuf filename;
  Combinator.run (tokens filename lexbuf) notations
;;

let parse_file (filename : string) : ENotation.notation list =
  let ch = open_in filename in
  Fun.protect ~finally:(fun _ -> close_in ch) @@ fun _ -> parse_channel filename ch
;;

(* This should not be invoke from outside, just for testing purpose *)
let parse_single (input : string) : ENotation.notation =
  let lexbuf = Sedlexing.Utf8.from_string input in
  Sedlexing.set_filename lexbuf "test";
  Combinator.run (tokens "test" lexbuf) enotation
;;

let%expect_test "identifier" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "x";
  [%expect {| x |}]
;;

let%expect_test "identifier weird" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "#%x";
  [%expect {| #%x |}]
;;

(* NOTE: `.` is not valid anymore, we preserve it for object accessing, therefore, the test case is a bit different from usual scheme *)
let%expect_test "identifier obsure" =
  print_string
  @@ [%show: ENotation.notation]
  @@ parse_single "obscure-name-!$%^&*-_=+<>/?";
  [%expect {| obscure-name-!$%^&*-_=+<>/? |}]
;;

let%expect_test "identifier 漢字" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "世界";
  [%expect {| 世界 |}]
;;

let%expect_test "identifier 日文" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "本好きの下剋上";
  [%expect {| 本好きの下剋上 |}]
;;

let%expect_test "identifier quote" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "|6|";
  [%expect {| |6| |}]
;;

let%expect_test "identifier many" =
  print_endline @@ [%show: ENotation.notation] @@ parse_single "λ";
  print_endline @@ [%show: ENotation.notation] @@ parse_single "😇";
  print_endline @@ [%show: ENotation.notation] @@ parse_single "ok#";
  [%expect
    {|
    λ
    😇
    ok#
    |}]
;;

let%expect_test "boolean true" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "#t";
  [%expect {| #t |}]
;;

let%expect_test "boolean false" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "#f";
  [%expect {| #f |}]
;;

let%expect_test "integer" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "1";
  [%expect {| 1 |}]
;;

let%expect_test "rational" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "1/2";
  [%expect {| 1/2 |}]
;;

let%expect_test "a comment then an identifier" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "#;(x y z) x";
  [%expect {| x |}]
;;

let%expect_test "list" =
  print_endline @@ [%show: ENotation.notation] @@ parse_single "(x y z)";
  print_endline @@ [%show: ENotation.notation] @@ parse_single "(1 2 3)";
  print_endline @@ [%show: ENotation.notation] @@ parse_single "[1 2 3]";
  print_endline @@ [%show: ENotation.notation] @@ parse_single "([1 2 3] 4 5 6)";
  [%expect
    {|
    (x y z)
    (1 2 3)
    (1 2 3)
    ((1 2 3) 4 5 6)
    |}]
;;

let%expect_test "vector" =
  print_endline @@ [%show: ENotation.notation] @@ parse_single "#(1 2 3)";
  print_endline @@ [%show: ENotation.notation] @@ parse_single "#(1 #(2 3))";
  [%expect
    {|
    #(1 2 3)
    #(1 #(2 3))
    |}]
;;
