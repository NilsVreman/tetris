pub struct Scoreboard {
    score: usize,
    fn_score: Box<dyn Fn(&usize) -> usize>,
}

impl Scoreboard {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&usize) -> usize + 'static
    {
        Self {
            score: 0,
            fn_score: Box::new(f),
        }
    }
}
