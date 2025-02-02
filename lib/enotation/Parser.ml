exception UnexpectedToken of { loc : Asai.Range.t }

let rec enotation () : ENotation.t =
  let open Combinator in
  let open ENotation in
  let tok = next_token () in
  let loc = Option.get tok.loc in
  match tok.value with
  | IDENTIFIER s -> Asai.Range.locate loc @@ Id s
  | OPEN_PAREN ->
    let notations = Combinator.many enotation () in
    consume Lexer.CLOSE_PAREN;
    Asai.Range.locate loc @@ L notations
  | NOTATION_COMMENT ->
    (* a #; comment will ignore next enotation, and take the next next enotaion *)
    let _ = enotation () in
    enotation ()
  | CLOSE_PAREN | EOF -> raise (UnexpectedToken { loc })
;;

let p_all () : ENotation.t list =
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

let parse_channel (filename : string) (ch : in_channel) =
  let lexbuf = Lexing.from_channel ch in
  lexbuf.lex_curr_p <- { lexbuf.lex_curr_p with pos_fname = filename };
  Combinator.run (tokens filename lexbuf) p_all
;;

let parse_file (filename : string) =
  let ch = open_in filename in
  Fun.protect ~finally:(fun _ -> close_in ch) @@ fun _ -> parse_channel filename ch
;;
