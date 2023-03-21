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

/// return a closure that returns the block's line at index `i`
pub fn rule_line_at_index(idx: usize, lines: &Vec<u16>) -> impl Fn(usize) -> u16 + '_ {
    move |i: usize| {
        match i {
            x if i >= idx && i < idx+4 => lines[i - idx],
            _ => 0,
        }
    }
}
