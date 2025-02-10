open Eio
open Ast
open Asai.Range
module Write = Eio.Buf_write

let ( / ) = Eio.Path.( / )

type mode =
  (* library mode only contains a list of definition, should have no computations *)
  | Library
  (* program mode contains computations, can be interactive from shell/GUI/... *)
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

let produce ~mode root (m : Expander.tapi_module) : 'a Eio.Path.t =
  let root = root / "_build" in
  Eio.Path.mkdirs ~exists_ok:true ~perm:0o777 root;
  let path = root / m.filename in
  prepare_file path;
  traceln "Saving to %a" Eio.Path.pp path;
  (* output path of xxx/yyy.ss is _build/xxx/yyy.ss *)
  Eio.Path.with_open_out ~append:false ~create:(`Or_truncate 0o777) path
  @@ fun out ->
  Write.with_flow out
  @@ fun w ->
  (match mode with
   | Library ->
     (* TODO: if m.program is not empty, we should raise an error in Library mode *)
     let module_name = parse_module m.filename in
     Write.printf w "(library (%s)\n" module_name;
     Write.printf w "  (export";
     Hashtbl.iter (fun name _ -> Write.printf w " %s" name) m.tops;
     Write.printf w ")\n";
     Write.printf w "  (import ";
     (match m.imports with
      | Some imports ->
        List.iter (fun i -> Write.printf w "(%s)" (parse_module i)) imports
      | None -> ());
     Write.string w ")\n\n";
     Hashtbl.iter
       (fun name { value = t; _ } ->
          Write.string w "  ";
          Write.printf w "(define %s %s)\n" name ([%show: term] t))
       m.tops;
     Write.string w ")\n"
   | Program ->
     Write.printf w "(import ";
     (match m.imports with
      | Some imports ->
        List.iter (fun i -> Write.printf w "(%s)" (parse_module i)) imports
      | None -> ());
     Write.string w ")\n\n";
     Hashtbl.iter
       (fun name { value = t; _ } ->
          Write.printf w "(define %s %s)\n" name ([%show: term] t))
       m.tops;
     Dynarray.iter
       (fun t ->
          Write.printf w "%s\n" ([%show: term] t);
          ())
       m.program;
     Write.string w "\n");
  path
;;
