(: x : u8)
(define x 200)

(pretty-print (u8+ x x)) ; 200 + 200 (around) = 144
(pretty-print (+ x x)) ; 200 + 200 = 400

(pretty-print (u8- 1)) ; -1 around to 255
(pretty-print (u8- 1 2)) ; -1 around to 255
(pretty-print (u8- 2 1)) ; 1
(pretty-print (u8* 1 2 3))
(pretty-print (u8/ 6 3)) ; 2
