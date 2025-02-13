(: x : string)
(define x "abc")
(: y : string)
(define y "def")

(pretty-print (string-append-immutable x y))
; truncate will modify string directly
(pretty-print (string-truncate! x 2))
(pretty-print (string-append-immutable x y))
(pretty-print (string->number "1.23"))
(pretty-print (string->number "123" 6))
