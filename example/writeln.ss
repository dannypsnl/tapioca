(writeln 10)

(let-values ([(sp g) (open-string-output-port)])
  (writeln 20 sp)
  (writeln (g)))
