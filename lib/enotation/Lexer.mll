{
exception TokenError of string

type token = 
  | OPEN_PAREN [@printer fun fmt () -> fprintf fmt "("]
  | CLOSE_PAREN [@printer fun fmt () -> fprintf fmt ")"]
  | NOTATION_COMMENT [@printer fun fmt () -> fprintf fmt "#;"]
  | IDENTIFIER of string [@printer fun fmt name -> fprintf fmt "%s" name]
  | EOF
[@@deriving_show]

let ident str = IDENTIFIER str
let return _lexbuf tok = tok
let illegal str = raise @@ TokenError str
}

let digit = ['0'-'9']
let alpha = ['a'-'z' 'A'-'Z']
let ident = (alpha) (alpha|digit|'_'|'-')*
let whitespace = [' ' '\t']+
let newline = '\r' | '\n' | "\r\n"

rule token =
  parse
  | ";" { comment lexbuf }
  | "#;" { return lexbuf @@ NOTATION_COMMENT }
  | '(' { return lexbuf @@ OPEN_PAREN }
  | ')' { return lexbuf @@ CLOSE_PAREN }
  | ident { return lexbuf @@ ident (Lexing.lexeme lexbuf) }
  | whitespace { token lexbuf }
  | newline { Lexing.new_line lexbuf; token lexbuf }
  | eof { EOF }
  | _ { illegal @@ Lexing.lexeme lexbuf }

and comment =
  parse
  | newline { Lexing.new_line lexbuf; token lexbuf }
  | eof { EOF }
  | _ { comment lexbuf }
