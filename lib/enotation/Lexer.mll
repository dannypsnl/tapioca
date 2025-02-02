{
exception TokenError of string

type token = 
  | OPEN_PAREN [@printer fun fmt () -> fprintf fmt "("]
  | CLOSE_PAREN [@printer fun fmt () -> fprintf fmt ")"]
  | NOTATION_COMMENT [@printer fun fmt () -> fprintf fmt "#;"]
  | IDENTIFIER of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | BOOL_TRUE  [@printer fun fmt () -> fprintf fmt "#t"]
  | BOOL_FALSE [@printer fun fmt () -> fprintf fmt "#f"]
  | INTEGER of int [@printer fun fmt i -> fprintf fmt "%s" (string_of_int i)]
  | EOF
[@@deriving show]

let ident text = IDENTIFIER text
let integer text = INTEGER (int_of_string text)
let return _lexbuf tok = tok
let illegal text = raise @@ TokenError text
}

let sign = '+'|'-'
let digit = ['0'-'9']
let integer = sign? digit+

let alpha = ['a'-'z' 'A'-'Z']
let ident = (alpha) (alpha|digit|'_'|'-')*
let whitespace = [' ' '\t']+
let newline = '\r' | '\n' | "\r\n"

rule token =
  parse
  | ";" { comment lexbuf }
  | "#;" { return lexbuf @@ NOTATION_COMMENT }
  | "#t" { return lexbuf @@ BOOL_TRUE }
  | "#f" { return lexbuf @@ BOOL_FALSE }
  | '(' { return lexbuf @@ OPEN_PAREN }
  | ')' { return lexbuf @@ CLOSE_PAREN }
  | ident { return lexbuf @@ ident (Lexing.lexeme lexbuf) }
  | integer { return lexbuf @@ integer (Lexing.lexeme lexbuf) }
  | whitespace { token lexbuf }
  | newline { Lexing.new_line lexbuf; token lexbuf }
  | eof { EOF }
  | _ { illegal @@ Lexing.lexeme lexbuf }

and comment =
  parse
  | newline { Lexing.new_line lexbuf; token lexbuf }
  | eof { EOF }
  | _ { comment lexbuf }
