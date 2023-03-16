# tetris
Tetris implemented in Rust

## Rules
* Board: 10 x 20 bricks (width x height)
* Blocks: I, J, L, O, S, T, Z
* Rotation: Clockwise and counter-clockwise. If there is no space for the piece
  in its new orientation, it does not rotate. Some games tolerate "wall kicks"
  or "wall nudges"

## Point system
Points are handed out when lines are removed:

```
Single line: 1 p
Double line: 3 p
Triple line: 8 p
Tetris:      20 p
```


## Cheat-Sheet

### Wall:
```
[0x3003, 0x3003, 0x3003, 0x3003]
0010 0000 0000 0100
0010 0000 0000 0100
0010 0000 0000 0100
0010 0000 0000 0100
```

### I-Block:
```
[0x0000, 0x0000, 0x0000, 0x03A0]
0000 0000 0000 0000
0000 0000 0000 0000
0000 0000 0000 0000
0000 0011 1100 0000
```

### J-Block:
```
[0x0000, 0x0000, 0x0200, 0x0360]
0000 0000 0000 0000
0000 0000 0000 0000
0000 0010 0000 0000
0000 0011 1000 0000
```

### L-Block:
```
[0x0000, 0x0000, 0x0060, 0x0360]
0000 0000 0000 0000
0000 0000 0000 0000
0000 0000 1000 0000
0000 0011 1000 0000
```

### O-Block:
```
[0x0000, 0x0000, 0x0160, 0x0160]
0000 0000 0000 0000
0000 0000 0000 0000
0000 0001 1000 0000
0000 0001 1000 0000
```

### S-Block:
```
[0x0000, 0x0000, 0x0160, 0x0300]
0000 0000 0000 0000
0000 0000 0000 0000
0000 0001 1000 0000
0000 0011 0000 0000
```

### T-Block:
```
[0x0000, 0x0000, 0x0100, 0x0360]
0000 0000 0000 0000
0000 0000 0000 0000
0000 0001 0000 0000
0000 0011 1000 0000
```

### Z-Block:
```
[0x0000, 0x0000, 0x0300, 0x0160]
0000 0000 0000 0000
0000 0000 0000 0000
0000 0011 0000 0000
0000 0001 1000 0000
```
