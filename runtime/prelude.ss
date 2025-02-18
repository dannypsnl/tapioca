(define home-dir (getenv "HOME"))
(load-shared-object (path-build home-dir ".tapioca/runtime/u8.so"))

(library (prelude)
  (export u8+)
  (import (chezscheme))


  (define u8_plus (foreign-procedure "u8_plus" (int u8*) unsigned-8))
  (define (u8+ . vs)
    (define vec (u8-list->bytevector vs))
    (u8_plus (bytevector-length vec) vec))

)
