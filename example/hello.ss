(import chezscheme)

(: x : int)
(define x 1)

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

(pretty-print x)
(pretty-print (f x y))
