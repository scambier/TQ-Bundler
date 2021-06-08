// title:  game title
// author: game developer
// desc:   short description
// script: wren

class Game is TIC{

	construct new(){
		_t=0
		_x=96
		_y=24
	}

	TIC(){
		if(TIC.btn(0)){
			_y=_y-1
		}
		if(TIC.btn(1)){
			_y=_y+1
		}
		if(TIC.btn(2)){
			_x=_x-1
		}
		if(TIC.btn(3)){
			_x=_x+1
		}

		TIC.cls(13)
		TIC.spr(1+((_t%60)/30|0)*2,_x,_y,14,3,0,0,2,2)
		TIC.print("HELLO WORLD!",84,84)

		_t=_t+1
	}
}