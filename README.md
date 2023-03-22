# tetris
Tetris implemented in Rust

## Rules
* Board: 10 x 20 bricks (width x height)
* Blocks: I, J, L, O, S, T, Z
* Rotation: Clockwise and counter-clockwise.
  If there is no space for the piece in its new orientation, it does not rotate.

## Point system
Points are handed out when lines are removed:

```
Single line: 1 p
Double line: 3 p
Triple line: 7 p
Tetris:      13 p
```


## Rotation Scheme

![alt text](rotation_scheme.png "Tetris rotations")
