(import chezscheme)

(: x : int)
(define x 3)

(: y : int)
(define y 2)

(: f : int int -> int)
(define (f a b)
  (let ([b a])
    b))

(: g : int int -> int)
(define g
  (lambda (a b)
    b))

(pretty-print (+ x y))
(pretty-print (- 4 3 2 1))
(pretty-print (* 2 (+ x y)))
(pretty-print (f x y))
