(import (chezscheme))

(define-record u8
  ([unsigned-8 value]))

(load-shared-object "./u8.so")

(define int-id
  (foreign-procedure "id" (int) int))

(define u8+
  (foreign-procedure "u8_plus" (int u8*) unsigned-8))

(define bv1 (make-bytevector 259 1))
(display
  (u8+ (bytevector-length bv1) bv1))

; This version is wrong, because don't implement u8 wrapping result
(let loop ([i 0] [out 0])
  (if (= i (bytevector-length bv1))
    (display out)
    (loop (add1 i) (fx+ out (bytevector-u8-ref bv1 i)))))
