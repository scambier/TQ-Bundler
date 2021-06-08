-- title:  game title
-- author: game developer
-- desc:   short description
-- script: moon

t=0
x=96
y=24

export TIC=->
	if btn 0
		y-=1
	if btn 1
		y+=1
	if btn 2
		x-=1
	if btn 3
		x+=1

	cls 13
	spr 1+(t%60)//30*2,x,y,14,3,0,0,2,2
	print "HELLO WORLD!",84,84
	t+=1
