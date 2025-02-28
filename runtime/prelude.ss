(define home-dir (getenv "HOME"))
(load-shared-object (path-build home-dir ".tapioca/runtime/tapioca.so"))

(library (prelude)
  (export
    u8+ u8- u8* u8/
    u16+ u16- u16* u16/
    writeln)
  (import (chezscheme))

  (define writeln
    (case-lambda
      [(x) (write x)
           (newline)]
      [(x p)
       (write x p)
       (newline p)]))

  (define u16_plus (foreign-procedure __collect_safe "u16_plus" (int u16*) unsigned-16))
  (define (u16+ . vs)
    (define vec (uint-list->bytevector vs (native-endianness) 2))
    (u16_plus (length vs) vec))

  (define u16_sub (foreign-procedure __collect_safe "u16_sub" (int unsigned-16 u16*) unsigned-16))
  (define (u16- v . vs)
    (define vec (uint-list->bytevector vs (native-endianness) 2))
    (u16_sub (length vs) v vec))

  (define u16_multiply (foreign-procedure __collect_safe "u16_multiply" (int u16*) unsigned-16))
  (define (u16* . vs)
    (define vec (uint-list->bytevector vs (native-endianness) 2))
    (u16_multiply (length vs) vec))

  (define u16_div (foreign-procedure __collect_safe "u16_div" (int unsigned-16 u16*) unsigned-16))
  (define (u16/ v . vs)
    (define vec (uint-list->bytevector vs (native-endianness) 2))
    (u16_div (length vs) v vec))

  (define u8_plus (foreign-procedure __collect_safe "u8_plus" (int u8*) unsigned-8))
  (define (u8+ . vs)
    (define vec (u8-list->bytevector vs))
    (u8_plus (bytevector-length vec) vec))

  (define u8_sub (foreign-procedure __collect_safe "u8_sub" (int unsigned-8 u8*) unsigned-8))
  (define (u8- v . vs)
    (define vec (u8-list->bytevector vs))
    (u8_sub (bytevector-length vec) v vec))

  (define u8_multiply (foreign-procedure __collect_safe "u8_multiply" (int u8*) unsigned-8))
  (define (u8* . vs)
    (define vec (u8-list->bytevector vs))
    (u8_multiply (bytevector-length vec) vec))

  (define u8_div (foreign-procedure __collect_safe "u8_div" (int unsigned-8 u8*) unsigned-8))
  (define (u8/ v . vs)
    (define vec (u8-list->bytevector vs))
    (u8_div (bytevector-length vec) v vec))

)
