open Cmdliner
open Tapioca_enotation
open Tapioca_expander
module Tty = Asai.Tty.Make (Tapioca_expander.Reporter.Message)

let version = "0.1.0"
let prelude_installed_path = Unix.getenv "HOME" ^ "/.tapioca/runtime"
let libraries = [ prelude_installed_path; "." ]

let load_primitive_types (ctx : Context.t) : unit =
  let filename = String.concat "/" [ prelude_installed_path; "prelude.sts" ] in
  let ns = Parser.parse_file filename in
  let m = Expander.expand_file filename ns in
  Context.import ctx m.context
;;

let compile ~env (mode : Chez.mode) (filename : string) : 'a Eio.Path.t =
  let root = Eio.Stdenv.cwd env in
  let ns = Parser.parse_file filename in
  let m = Expander.expand_file filename ns in
  load_primitive_types m.context;
  Tyck.check_module m;
  Chez.produce ~mode root m
;;

let compile_cmd ~env =
  let _ = env in
  let arg_file =
    let doc = "The program file to compile." in
    Arg.required @@ Arg.pos 0 (Arg.some Arg.file) None @@ Arg.info [] ~docv:"PROG" ~doc
  in
  let is_program_mode =
    let doc = "" in
    Arg.value @@ Arg.flag @@ Arg.info [ "program" ] ~doc
  in
  let doc = "Compile input program file to chez scheme" in
  let man = [ `S Manpage.s_description; `P "" ] in
  let info = Cmd.info "compile" ~version ~doc ~man in
  Cmd.v
    info
    Term.(
      const (fun is_program_mode file ->
        let mode = if is_program_mode then Chez.Program else Chez.Library in
        let _ = compile ~env mode file in
        ())
      $ is_program_mode
      $ arg_file)
;;

let run_cmd ~env =
  let _ = env in
  let arg_file =
    let doc = "The program file to run." in
    Arg.required @@ Arg.pos 0 (Arg.some Arg.file) None @@ Arg.info [] ~docv:"PROG" ~doc
  in
  let doc = "Run input program file" in
  let man = [ `S Manpage.s_description; `P "" ] in
  let info = Cmd.info "run" ~version ~doc ~man in
  Cmd.v
    info
    Term.(
      const (fun file ->
        let output = compile ~env Program file in
        let proc_mgr = Eio.Stdenv.process_mgr env in
        let path : string = Format.sprintf "%s" @@ Option.get @@ Eio.Path.native output in
        Eio.Process.run
          proc_mgr
          [ "scheme"; "--libdirs"; String.concat ":" libraries; "--program"; path ];
        ())
      $ arg_file)
;;

let load_cmd ~env =
  let _ = env in
  let arg_file =
    let doc = "The program file to load." in
    Arg.required @@ Arg.pos 0 (Arg.some Arg.file) None @@ Arg.info [] ~docv:"PROG" ~doc
  in
  let doc = "Load input program file into REPL" in
  let man = [ `S Manpage.s_description; `P "" ] in
  let info = Cmd.info "load" ~version ~doc ~man in
  Cmd.v info Term.(const (fun _filename -> ()) $ arg_file)
;;

let cmd ~env =
  let doc = "tapioca" in
  let man = [ `S Manpage.s_bugs; `S Manpage.s_authors; `P "Lîm Tsú-thuàn" ] in
  let info = Cmd.info "tapioca" ~version ~doc ~man in
  Cmd.group info [ compile_cmd ~env; run_cmd ~env; load_cmd ~env ]
;;

let () =
  let fatal diagnostics =
    Tty.display diagnostics;
    exit 1
  in
  Printexc.record_backtrace true;
  Eio_main.run
  @@ fun env ->
  Tapioca_expander.Reporter.run ~emit:Tty.display ~fatal
  @@ fun () -> exit @@ Cmd.eval ~catch:false @@ cmd ~env
;;
