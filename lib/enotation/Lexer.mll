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
  | RATIONAL of int * int [@printer fun fmt (p, q) -> fprintf fmt "%s/%s" (string_of_int p) (string_of_int q)]
  | EOF
[@@deriving show]

let ident text = IDENTIFIER text
let integer text = INTEGER (int_of_string text)
let rational text : token =
  let s : string list = String.split_on_char '/' text in
  match s with
  | p :: q :: [] -> RATIONAL ((int_of_string p), (int_of_string q))
  | _ -> raise @@ TokenError "rational form is p/q"
let return _lexbuf tok = tok
let illegal text = raise @@ TokenError text
}

let sign = '+'|'-'
let digit = ['0'-'9']
let integer = sign? digit+

let rational = sign? digit+ "/" digit+

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
  | rational { return lexbuf @@ rational (Lexing.lexeme lexbuf) }
  | whitespace { token lexbuf }
  | newline { Lexing.new_line lexbuf; token lexbuf }
  | eof { EOF }
  | _ { illegal @@ Lexing.lexeme lexbuf }

and comment =
  parse
  | newline { Lexing.new_line lexbuf; token lexbuf }
  | eof { EOF }
  | _ { comment lexbuf }
