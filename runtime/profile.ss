(parameterize ([compile-profile 'source])
  (load "./u8.ss"))

(let loop ([i 0])
  (if (= i 10000)
    (void)
    (begin
      (a)
      (loop (add1 i)))))
(display (statistics))
