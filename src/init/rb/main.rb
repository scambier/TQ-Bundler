# title:  game title
# author: game developer
# desc:   short description
# script: ruby

$t=0
$x=96
$y=24

def TIC
	$y-=1 if btn 0
	$y+=1 if btn 1
	$x-=1 if btn 2
	$x+=1 if btn 3

	cls 13
	spr 1+(($t%60)/30|0)*2,$x,$y,14,3,0,0,2,2
	print "HELLO WORLD!",84,84
	$t+=1
end
