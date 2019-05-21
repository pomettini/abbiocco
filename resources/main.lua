frame = 0

function _init()
end

function _update()
	frame = frame + 1
end

function _draw()
	cls()
	rect(frame, frame, frame + 100, frame + 100, 1)
	rectfill(frame + 50, frame + 50, frame + 150, frame + 150, 2)
end

-- spr(1, 60, 60)
