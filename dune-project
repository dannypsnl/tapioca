(lang dune 3.17)

(name tapioca)

(version 0.1.0)

(generate_opam_files true)

(source
 (github dannypsnl/tapioca))

(authors "Lîm Tsú-thuàn <inbox@dannypsnl.me>")

(maintainers "Lîm Tsú-thuàn <inbox@dannypsnl.me>")

(license LICENSE)

(documentation https://github.com/dannypsnl/tapioca)

(package
 (name tapioca)
 (synopsis "A short synopsis")
 (description "A longer description")
 (depends
  ocaml
  dune
  ocamlformat
  ppxlib
  ppx_expect
  ppx_deriving
  sedlex
  cmdliner
  eio
  eio_main
  asai
  algaeff
  bwd)
 (tags
  ("tapioca" "scheme")))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/reference/dune-project/index.html
