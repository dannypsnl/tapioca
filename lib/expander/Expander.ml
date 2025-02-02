open Tapioca_enotation.ENotation

exception BadImport
exception BadForm of notation

type top = Import of string list

let expand_file (_ns : notation list) = ()

let expand_top (n : notation) =
  match n with
  | L (Id "import" :: modules) ->
    let modules =
      List.map
        (fun n ->
           match n with
           | Id n -> n
           | _ -> raise BadImport)
        modules
    in
    Import modules
  | _ -> raise @@ BadForm n
;;
