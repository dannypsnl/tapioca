open Sedlexing

type token =
  | OPEN_VECTOR [@printer fun fmt () -> fprintf fmt "#("]
  | OPEN_PAREN [@printer fun fmt () -> fprintf fmt "("]
  | CLOSE_PAREN [@printer fun fmt () -> fprintf fmt ")"]
  | OPEN_BRACKET [@printer fun fmt () -> fprintf fmt "["]
  | CLOSE_BRACKET [@printer fun fmt () -> fprintf fmt "]"]
  | DOTS [@printer fun fmt () -> fprintf fmt "..."]
  | NOTATION_COMMENT [@printer fun fmt () -> fprintf fmt "#;"]
  | IDENTIFIER of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | BOOL_TRUE [@printer fun fmt () -> fprintf fmt "#t"]
  | BOOL_FALSE [@printer fun fmt () -> fprintf fmt "#f"]
  | INTEGER of int [@printer fun fmt i -> fprintf fmt "%s" (string_of_int i)]
  | FLOAT of float [@printer fun fmt f -> fprintf fmt "%s" (string_of_float f)]
  | STRING of string [@printer fun fmt s -> fprintf fmt "\"%s\"" s]
  | RATIONAL of int * int
  [@printer fun fmt (p, q) -> fprintf fmt "%s/%s" (string_of_int p) (string_of_int q)]
  | EOF
[@@deriving show]

exception LexError of Lexing.position * string

let blank = [%sedlex.regexp? ' ' | '\t']
let newline = [%sedlex.regexp? '\r' | '\n']
let whitespace = [%sedlex.regexp? Plus (blank | newline)]
let decimal_ascii = [%sedlex.regexp? Plus '0' .. '9']
let octal_ascii = [%sedlex.regexp? "0o", Plus '0' .. '7']
let hex_ascii = [%sedlex.regexp? "0x", Plus ('0' .. '9' | 'a' .. 'f' | 'A' .. 'F')]

let idChunk =
  [%sedlex.regexp?
    ( ( Compl
          ( ' '
          | '\t'
          | '\n'
          | '\r'
          | '('
          | ')'
          | '['
          | ']'
          | '{'
          | '}'
          | '"'
          | ','
          | '\''
          | '`'
          | ';'
          | '#'
          | '|'
          | '\\'
          | '.' )
      | "#%" )
    , Star
        (Compl
           ( ' '
           | '\t'
           | '\n'
           | '\r'
           | '('
           | ')'
           | '['
           | ']'
           | '{'
           | '}'
           | '"'
           | ','
           | '\''
           | '`'
           | ';'
           | '|'
           | '\\'
           | '.' )) )]
;;

let stringChunk = [%sedlex.regexp? Star (Compl ('"' | '\\' | '\n'))]

let rec skipWhitespace buf =
  match%sedlex buf with
  | Plus whitespace -> skipWhitespace buf
  | _ -> ()
;;

let comment buf =
  match%sedlex buf with
  | Star (Compl newline), newline -> ()
  | _ -> assert false
;;

let multiline_comment buf =
  match%sedlex buf with
  | Star (Compl (Chars "|#")), "|#" -> ()
  | _ -> assert false
;;

let num_value buffer =
  let buf = Utf8.lexeme buffer in
  int_of_string buf
;;

let rec token buf =
  skipWhitespace buf;
  match%sedlex buf with
  | eof -> EOF
  | "#|" ->
    multiline_comment buf;
    token buf
  | ";" ->
    comment buf;
    token buf
  | newline ->
    comment buf;
    token buf
  | "#t" -> BOOL_TRUE
  | "#f" -> BOOL_FALSE
  | "#;" -> NOTATION_COMMENT
  | "#(" -> OPEN_VECTOR
  | '(' -> OPEN_PAREN
  | ')' -> CLOSE_PAREN
  | '[' -> OPEN_BRACKET
  | ']' -> CLOSE_BRACKET
  | "..." -> DOTS
  | '"', stringChunk, '"' ->
    let buff = Utf8.lexeme buf in
    STRING buff
  | decimal_ascii, '.', decimal_ascii ->
    let buff = Utf8.lexeme buf in
    FLOAT (float_of_string buff)
  | decimal_ascii, '/', decimal_ascii -> begin
    let buff = Utf8.lexeme buf in
    match String.split_on_char '/' buff with
    | [ p; q ] -> RATIONAL (int_of_string p, int_of_string q)
    | _ ->
      let loc = Asai.Range.of_lex_range (Sedlexing.lexing_bytes_positions buf) in
      LexReporter.fatalf Lex_error ~loc "rational number form should be p/q"
  end
  | '-', decimal_ascii ->
    let number = num_value buf in
    INTEGER number
  | decimal_ascii ->
    let number = num_value buf in
    INTEGER number
  | '|', Star (Compl '|'), '|' -> IDENTIFIER (Utf8.lexeme buf)
  | idChunk -> IDENTIFIER (Utf8.lexeme buf)
  | _ ->
    let _ = Sedlexing.next buf in
    (* Skip the bad character: pretend it's a token *)
    let tok = Utf8.lexeme buf in
    let loc = Asai.Range.of_lex_range (Sedlexing.lexing_bytes_positions buf) in
    LexReporter.fatalf Lex_error ~loc "Unexpected character: %s" tok
;;

let lexer buf = Sedlexing.with_tokenizer token buf
