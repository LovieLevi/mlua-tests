Speed = 5 * 60 -- 5 pixels per second at 60 FPS
Delimiter = 1

function Update()
  local player_pos = Player:get_position()

  local angle = Time * math.pi / 2
  if angle > 2 * math.pi then
    angle = angle - 2 * math.pi
  end

  local distance = ((Time + 2) ^ 2) - Delimiter

  local max_distance = ((ScreenWidth / 2) ^ 2 + (ScreenHeight / 2) ^ 2) ^ 0.5 + 25
  if distance > max_distance then
    Delimiter = Delimiter + max_distance
  end

  -- Convert radian coordinates to cartesian coordinates
  player_pos.x = ScreenWidth / 2 + distance * math.cos(angle)
  player_pos.y = ScreenHeight / 2 + distance * math.sin(angle)

  Player:set_position(player_pos)
end

