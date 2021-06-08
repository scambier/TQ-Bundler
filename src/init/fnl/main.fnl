;; title:  game title
;; author: game developer
;; desc:   short description
;; script: fennel

(var t 0)
(var x 96)
(var y 24)

(global TIC
 (fn tic []
  (when (btn 0) (set y (- y 1)))
  (when (btn 1) (set y (+ y 1)))
  (when (btn 2) (set x (- x 1)))
  (when (btn 3) (set x (+ x 1)))
  (cls 0)
  (spr (+ 1 (* (// (% t 60) 30) 2))
       x y 14 3 0 0 2 2)
  (print "HELLO WORLD!" 84 84)
  (set t (+ t 1))))