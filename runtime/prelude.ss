(define home-dir (getenv "HOME"))
(load-shared-object (path-build home-dir ".tapioca/runtime/tapioca.so"))

(library (prelude)
  (export u8+ u8- u8* u8/)
  (import (chezscheme))


  (define u8_plus (foreign-procedure "u8_plus" (int u8*) unsigned-8))
  (define (u8+ . vs)
    (define vec (u8-list->bytevector vs))
    (u8_plus (bytevector-length vec) vec))

  (define u8_sub (foreign-procedure "u8_sub" (int unsigned-8 u8*) unsigned-8))
  (define (u8- . vs)
    (define vec (u8-list->bytevector (cdr vs)))
    (u8_sub (bytevector-length vec) (car vs) vec))

  (define u8_multiply (foreign-procedure "u8_multiply" (int u8*) unsigned-8))
  (define (u8* . vs)
    (define vec (u8-list->bytevector vs))
    (u8_multiply (bytevector-length vec) vec))

  (define u8_div (foreign-procedure "u8_div" (int unsigned-8 u8*) unsigned-8))
  (define (u8/ . vs)
    (define vec (u8-list->bytevector (cdr vs)))
    (u8_div (bytevector-length vec) (car vs) vec))

)
