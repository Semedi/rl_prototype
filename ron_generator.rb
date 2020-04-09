ROWS        = 32
COLUMNS     = 32
WIDTH       = 543
HEIGHT      = 543
SEPARATOR   = 1
TILE_WIDTH  = 16
TILE_HEIGHT = 16

##########################################################

def tab(level, chain)
  ttab = "    "
  r    = ""
  for i in 0..level do 
    r << ttab
  end

  r << chain << "\n"
end

config = <<-EOF
#![enable(implicit_some)]
List((
    texture_width: #{WIDTH},
    texture_height: #{HEIGHT},
    // List of sprites the sheet holds
    sprites: [
EOF

offx = 0
offy = 0
for y in 0...COLUMNS do 
  offx = 0
  for x in 0...ROWS do 
    config << tab(2, '(')
    config << tab(3, "x: #{offx},")
    config << tab(3, "y: #{offy},")
    config << tab(3, "width: #{TILE_WIDTH},")
    config << tab(3, "height: #{TILE_HEIGHT},")
    config << tab(2, '),')
    offx += TILE_WIDTH + SEPARATOR
  end
  offy += TILE_HEIGHT + 1
end

config << <<-EOF
    ],
))
EOF

print config


