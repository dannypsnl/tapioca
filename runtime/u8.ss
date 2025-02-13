(import (chezscheme))

(define-record u8
  ([unsigned-8 value]))

(define (u8+ a b)
  (make-u8
    (fxmodulo (fx+ (u8-value a) (u8-value b))
              256)))

;(display (make-u8 255))
(display (u8+ (make-u8 255) (make-u8 1)))

