cur_frame = 0

function _init()
    
end

function _update()
	cur_frame = cur_frame + 1
end

function _draw()
	rectfill(10 + cur_frame, 10 + cur_frame, 50, 50, 1)
end

-- spr(1, 60, 60)
