# title:   game title
# author:  game developer, email, etc.
# desc:    short description
# site:    website link
# license: MIT License (change this to your license of choice)
# version: 0.1
# script: janet
# strict:  true

# Unlike other langauges, the tic80 API
# is provided as a module.
(import tic80)

(var t 0)
(var x 96)
(var y 24)

(defn TIC []
  (when (tic80/btn 0) (-- y))
  (when (tic80/btn 1) (++ y))
  (when (tic80/btn 2) (-- x))
  (when (tic80/btn 3) (++ x))
  (tic80/cls 13)
  (tic80/spr (if (> (% t 60) 30) 1 3)
          x y 14 3 0 0 2 2)
  (tic80/print "HELLO WORLD!" 84 84)
  (++ t))