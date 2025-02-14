(import (chezscheme))

(define-record u8
  ([unsigned-8 value]))

(load-shared-object "./u8.so")

(define int-id
  (foreign-procedure "id" (int) int))

(define u8+
  (foreign-procedure "u8_plus" (int u8*) unsigned-8))

(define bv1 (make-bytevector 255 1))
(define (a)
  (u8+ (bytevector-length bv1) bv1))

(define (b)
  (let loop ([i 0] [out 0])
  (if (= i (bytevector-length bv1))
    out
    (loop (add1 i) (fx+ out (bytevector-u8-ref bv1 i))))
  ))
