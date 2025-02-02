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
    | OPEN_PAREN ->
      let notations = Combinator.many enotation () in
      consume Lexer.CLOSE_PAREN;
      L notations
    | NOTATION_COMMENT ->
      (* a #; comment will ignore next enotation, and take the next next enotaion *)
      let _ = enotation () in
      enotation ()
    | CLOSE_PAREN -> raise CloseParen
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
  let loc = Asai.Range.of_lexbuf ~source:(`File filename) lexbuf in
  match tok with
  | EOF -> Asai.Range.locate loc tok :: []
  | tok -> Asai.Range.locate loc tok :: tokens filename lexbuf
;;

let parse_channel (filename : string) (ch : in_channel) : ENotation.notation list =
  let lexbuf = Lexing.from_channel ch in
  lexbuf.lex_curr_p <- { lexbuf.lex_curr_p with pos_fname = filename };
  Combinator.run (tokens filename lexbuf) notations
;;

let parse_file (filename : string) : ENotation.notation list =
  let ch = open_in filename in
  Fun.protect ~finally:(fun _ -> close_in ch) @@ fun _ -> parse_channel filename ch
;;

(* This should not be invoke from outside, just for testing purpose *)
let parse_single (input : string) : ENotation.notation =
  let lexbuf = Lexing.from_string input in
  lexbuf.lex_curr_p <- { lexbuf.lex_curr_p with pos_fname = "test" };
  Combinator.run (tokens "test" lexbuf) enotation
;;

let%expect_test "single identifier" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "x";
  [%expect {| x |}]
;;

let%expect_test "a list of identifier" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "(x y z)";
  [%expect {| (x y z) |}]
;;

let%expect_test "a comment then an identifier" =
  print_string @@ [%show: ENotation.notation] @@ parse_single "#;(x y z) x";
  [%expect {| x |}]
;;
