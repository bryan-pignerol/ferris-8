POS_X = 64
POS_Y = 64
SPD = 50

function Ready()
    pset(pos_x, POS_Y, BLUE)
    rect(32, 32, 96, 96, RED)
    --line(32, 32, 96, 96, 0xFF0000)
end

function Update(dt)
    if btn(UP) then
        POS_Y = POS_Y - (SPD * dt)
    elseif btn(DOWN) then
        POS_Y = POS_Y + (SPD * dt)
    elseif btn(LEFT) then
        POS_X = POS_X - (SPD * dt)
    elseif btn(RIGHT) then
        POS_X = POS_X + (SPD * dt)
    end
end

function Draw(dt)
    clr(DARK_BLUE)
    pset(POS_X, POS_Y, BLUE)
    rect(32, 32, 96, 96, RED)
    --line(32, 32, 96, 96, 0xFF0000)
    txt("12345\n67890", 16, 16, RED)
end
