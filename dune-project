(lang dune 3.17)

(name tapioca)

(generate_opam_files true)

(source
 (github username/reponame))

(authors "Author Name <author@example.com>")

(maintainers "Maintainer Name <maintainer@example.com>")

(license LICENSE)

(documentation https://url/to/documentation)

(package
 (name tapioca)
 (synopsis "A short synopsis")
 (description "A longer description")
 (depends
  ocaml
  (cmdliner
   (>= 1.2.0))
  (eio_main
   (>= 1.1))
  (ptime
   (>= 1.1.0))
  (asai
   (>= 0.3.0))
  (algaeff
   (>= 2.0.0)))
 (tags
  ("add topics" "to describe" your project)))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/reference/dune-project/index.html
