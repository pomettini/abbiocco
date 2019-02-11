frame = 0

function _init()
end

function _update()
	frame = frame + 1
end

function _draw()
	cls()
	rect(0, 0, 100, 100, 1)
	rectfill(50, 50, 150, 150, 2)
end

-- spr(1, 60, 60)
