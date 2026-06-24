POS_X = 64
POS_Y = 64

function Ready()
    pset(pos_x, POS_Y, 0x0000FF)
    pset(64, 64, 0xFF0000)
    rect(32, 32, 96, 96, 0xFF0000)
    --line(32, 32, 96, 96, 0xFF0000)
end

function Update()
    if btn(0) then
        POS_Y = POS_Y - 1
    elseif btn(1) then
        POS_Y = POS_Y + 1
    elseif btn(2) then
        POS_X = POS_X - 1
    elseif btn(3) then
        POS_X = POS_X + 1
    end
end

function Draw()
    clr(0x000000)
    pset(POS_X, POS_Y, 0x0000FF)
    rect(32, 32, 96, 96, 0xFF0000)
    --line(32, 32, 96, 96, 0xFF0000)
end
