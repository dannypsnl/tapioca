open Eio
open Ast
module Write = Eio.Buf_write

let ( / ) = Eio.Path.( / )

type mode =
  | Library
  | Program

let parse_module (filename : string) : string =
  let n = String.split_on_char '.' filename in
  let name = List.hd n in
  String.concat " " @@ String.split_on_char '/' name
;;

let prepare_file path =
  match Eio.Path.split path with
  | Some (dir, _) -> Eio.Path.mkdirs ~exists_ok:true ~perm:0o777 dir
  | None -> ()
;;

let produce ~mode root (m : Expander.tapi_module) : unit =
  let root = root / "_build" in
  Eio.Path.mkdirs ~exists_ok:true ~perm:0o777 root;
  let path = root / m.filename in
  prepare_file path;
  traceln "Saving to %a" Eio.Path.pp path;
  Eio.Path.with_open_out ~append:false ~create:(`Or_truncate 0o777) path
  @@ fun out ->
  Write.with_flow out
  @@ fun w ->
  match mode with
  | Library ->
    let module_name = parse_module m.filename in
    Write.printf w "(library (%s)\n" module_name;
    Write.printf w "  (export";
    Hashtbl.iter (fun name _ -> Write.printf w " %s" name) m.tops;
    Write.printf w ")\n";
    Write.printf w "  (import ";
    (match m.imports with
     | Some imports -> List.iter (fun i -> Write.printf w "(%s)" (parse_module i)) imports
     | None -> ());
    Write.string w ")\n\n";
    Hashtbl.iter
      (fun name t ->
         Write.string w "  ";
         Write.printf w "(define %s %s)\n" name ([%show: term] t))
      m.tops;
    Write.string w ")\n"
  | Program ->
    Write.printf w "(import ";
    (match m.imports with
     | Some imports -> List.iter (fun i -> Write.printf w "(%s)" (parse_module i)) imports
     | None -> ());
    Write.string w ")\n\n";
    Hashtbl.iter
      (fun name t -> Write.printf w "(define %s %s)\n" name ([%show: term] t))
      m.tops;
    Write.string w "\n"
;;
