module Message = struct
  type t = Lex_error [@@deriving show]

  let default_severity : t -> Asai.Diagnostic.severity = function
    | Lex_error -> Error
  ;;

  let short_code : t -> string = show
end

include Asai.Reporter.Make (Message)
