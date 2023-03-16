# tetris
Tetris implemented in Rust

## Rules
* Board: 10 x 20 bricks (width x height)
* Blocks: I, J, L, O, S, T, Z
* Rotation: Clockwise and counter-clockwise. If there is no space for the piece
  in its new orientation, it does not rotate. Some games tolerate "wall kicks"
  or "wall nudges"
