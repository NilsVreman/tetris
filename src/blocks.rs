use std::fmt;

enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub struct Block {
    lines: u64,
    blocktype: BlockType,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        (0..64).rev().for_each(|i| {
            s.push_str(&format!("{}", if self.lines & (1 << i) != 0 { 1 } else { 0 })[..]);
            if i % 16 == 0 && i != 0 {
                s.push_str("\n");
            }
        });
        write!(f, "{}", s)
    }
}

impl Block {
    
}

pub const BLOCKS: [Block; 7] = [
    Block {
        lines: 0x00000000000003C0,
        blocktype: BlockType::I,
    },
    Block {
        lines: 0x0000000002000380,
        blocktype: BlockType::J,
    },
    Block {
        lines: 0x0000000000800380,
        blocktype: BlockType::L,
    },
    Block {
        lines: 0x0000000001800180,
        blocktype: BlockType::O,
    },
    Block {
        lines: 0x0000000001800300,
        blocktype: BlockType::S,
    },
    Block {
        lines: 0x0000000001000380,
        blocktype: BlockType::T,
    },
    Block {
        lines: 0x0000000003000180,
        blocktype: BlockType::Z,
    },
];

// Cheat-Sheet
// Wall: 0x3003300330033003
// 0010 0000 0000 0100
// 0010 0000 0000 0100
// 0010 0000 0000 0100
// 0010 0000 0000 0100
//
// I-Block: 0x00000000000003A0
// 0000 0000 0000 0000
// 0000 0000 0000 0000
// 0000 0000 0000 0000
// 0000 0011 1100 0000
//
// J-Block: 0x0000000002000360
// 0000 0000 0000 0000
// 0000 0000 0000 0000
// 0000 0010 0000 0000
// 0000 0011 1000 0000
//
// L-Block: 0x0000000000600360
// 0000 0000 0000 0000
// 0000 0000 0000 0000
// 0000 0000 1000 0000
// 0000 0011 1000 0000
//
// O-Block: 0x0000000001600160
// 0000 0000 0000 0000
// 0000 0000 0000 0000
// 0000 0001 1000 0000
// 0000 0001 1000 0000
//
// S-Block: 0x0000000001600300
// 0000 0000 0000 0000
// 0000 0000 0000 0000
// 0000 0001 1000 0000
// 0000 0011 0000 0000
//
// T-Block: 0x0000000001000360
// 0000 0000 0000 0000
// 0000 0000 0000 0000
// 0000 0001 0000 0000
// 0000 0011 1000 0000
//
// Z-Block: 0x0000000003000160
// 0000 0000 0000 0000
// 0000 0000 0000 0000
// 0000 0011 0000 0000
// 0000 0001 1000 0000
