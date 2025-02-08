open Sedlexing

type token =
  | OPEN_VECTOR [@printer fun fmt () -> fprintf fmt "#("]
  | OPEN_PAREN [@printer fun fmt () -> fprintf fmt "("]
  | CLOSE_PAREN [@printer fun fmt () -> fprintf fmt ")"]
  | OPEN_BRACKET [@printer fun fmt () -> fprintf fmt "["]
  | CLOSE_BRACKET [@printer fun fmt () -> fprintf fmt "]"]
  | NOTATION_COMMENT [@printer fun fmt () -> fprintf fmt "#;"]
  | IDENTIFIER of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | BOOL_TRUE [@printer fun fmt () -> fprintf fmt "#t"]
  | BOOL_FALSE [@printer fun fmt () -> fprintf fmt "#f"]
  | INTEGER of int [@printer fun fmt i -> fprintf fmt "%s" (string_of_int i)]
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

let comment echo buf =
  match%sedlex buf with
  | Star (Compl newline), newline ->
    if echo then Format.fprintf Format.std_formatter "%s%!" (Utf8.lexeme buf) else ()
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
  | ";" ->
    comment true buf;
    token buf
  | newline ->
    comment false buf;
    token buf
  | "#|" ->
    comment true buf;
    token buf
  | "|#" ->
    comment false buf;
    token buf
  | "#t" -> BOOL_TRUE
  | "#f" -> BOOL_FALSE
  | "#;" -> NOTATION_COMMENT
  | "#(" -> OPEN_VECTOR
  | '(' -> OPEN_PAREN
  | ')' -> CLOSE_PAREN
  | '[' -> OPEN_BRACKET
  | ']' -> CLOSE_BRACKET
  | '"', stringChunk, '"' ->
    let buff = Utf8.lexeme buf in
    STRING buff
  | decimal_ascii, '/', decimal_ascii ->
    let buff = Utf8.lexeme buf in
    (match String.split_on_char '/' buff with
     | [ p; q ] -> RATIONAL (int_of_string p, int_of_string q)
     | _ ->
       let pos = fst @@ lexing_positions buf in
       raise @@ LexError (pos, "rational number form should be p/q"))
  | decimal_ascii ->
    let number = num_value buf in
    INTEGER number
  | '|', Star (Compl '|'), '|' -> IDENTIFIER (Utf8.lexeme buf)
  | idChunk -> IDENTIFIER (Utf8.lexeme buf)
  | _ ->
    let pos = fst @@ lexing_positions buf in
    let _ = Sedlexing.next buf in
    (* Skip the bad character: pretend it's a token *)
    let tok = Utf8.lexeme buf in
    raise @@ LexError (pos, "Unexpected character: " ^ tok)
;;

let lexer buf = Sedlexing.with_tokenizer token buf
