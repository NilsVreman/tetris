/// How much the score should change depending on the number of lines cleared
pub fn rule_score() -> impl Fn(&usize) -> usize {
    |x: &usize| match x {
        1 => 1,
        2 => 3,
        3 => 7,
        4 => 13,
        _ => 0
    }
}

/// How much the score should change depending on the number of lines cleared
pub fn rule_nextblock() -> impl Fn(&usize) -> usize {
    |x: &usize| (x+1) % 7
}

use crate::block;

/// return a closure that returns the block's line at index `i`
pub fn rule_blockline_at_index<'a>(idx: usize, block: &'a block::Block) -> impl Fn(usize) -> &'a u16 {
    let lines = block.config();
    move |i: usize| {
        match i {
            x if i >= idx && i < idx+4 => &lines[i - idx],
            _ => &0,
        }
    }
}
