open Cmdliner
module Tty = Asai.Tty.Make (Tapioca_expander.Reporter.Message)

let version = "0.1.0"

let compile filename : unit =
  let ns = Tapioca_enotation.Parser.parse_file filename in
  let m = Tapioca_expander.Expander.expand_file ns in
  let _ = m in
  (* TODO: check type of m *)
  (* TODO: dump m to chez *)
  ()
;;

let compile_cmd ~env =
  let _ = env in
  let arg_file =
    let doc = "The program file to compile." in
    Arg.required @@ Arg.pos 0 (Arg.some Arg.file) None @@ Arg.info [] ~docv:"PROG" ~doc
  in
  let doc = "Compile input program file to chez scheme" in
  let man = [ `S Manpage.s_description; `P "" ] in
  let info = Cmd.info "compile" ~version ~doc ~man in
  Cmd.v info Term.(const compile $ arg_file)
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
  Cmd.v
    info
    Term.(
      const (fun filename ->
        let ns = Tapioca_enotation.Parser.parse_file filename in
        let m = Tapioca_expander.Expander.expand_file ns in
        let _ = m in
        ())
      $ arg_file)
;;

let cmd ~env =
  let doc = "tapioca" in
  let man = [ `S Manpage.s_bugs; `S Manpage.s_authors; `P "Lîm Tsú-thuàn" ] in
  let info = Cmd.info "tapioca" ~version ~doc ~man in
  Cmd.group info [ compile_cmd ~env; load_cmd ~env ]
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
