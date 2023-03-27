const SCORE_0_LINE: usize = 0;
const SCORE_1_LINE: usize = 1;
const SCORE_2_LINE: usize = 3;
const SCORE_3_LINE: usize = 7;
const SCORE_4_LINE: usize = 13;

pub struct Scoreboard {
    score: usize,
}

impl Scoreboard {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn update_score(&mut self, lines_cleared: usize) {
        self.score += match lines_cleared {
            0 => SCORE_0_LINE,
            1 => SCORE_1_LINE,
            2 => SCORE_2_LINE,
            3 => SCORE_3_LINE,
            4 => SCORE_4_LINE,
            _ => panic!("Not implemented more than four line tetris"),
        };
    }

    pub fn get_score(&self) -> usize {
        self.score
    }
}
