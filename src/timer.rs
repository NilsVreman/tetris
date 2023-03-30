// Chrono is the WASM safe timing library if used with features ["js-sys", "wasmbind"]
use chrono;

struct TickPeriod {
    min_period: i64,
    period: i64,  // Update frequency in milliseconds (ms)
    next_lvl: i64,
    lvl_up: i64,
}

impl TickPeriod {
    fn new(period: i64, min_period: i64, lvl_up: i64) -> Self {
        Self { period, min_period, lvl_up, next_lvl: lvl_up }
    }

    fn decrease_period(&mut self) {
        if self.period / 2  > self.min_period {
            self.period /= 2;
        }
    }

    fn update_period_from_score(&mut self, score: usize) {
        if score as i64 >= self.next_lvl {
            self.next_lvl += self.lvl_up;
            self.decrease_period();
        }
    }

    fn get_period(&self) -> i64 {
        self.period
    }
}

pub struct TickTimer {
    period: TickPeriod,
    time: i64,
}

impl TickTimer {
    pub fn new(period: i64, min_period: i64, lvl_up: i64) -> Self {
        let time = chrono::Local::now().timestamp_millis();
        Self {
            period: TickPeriod::new(period, min_period, lvl_up),
            time
        }
    }

    pub fn update_period_from_score(&mut self, score: usize) {
        self.period.update_period_from_score(score);
    }

    pub fn get_time_until_tick(&self) -> i64 {
        (self.time + self.period.get_period()) - chrono::Local::now().timestamp_millis()
    }

    pub fn reset_tick(&mut self) {
        self.time = chrono::Local::now().timestamp_millis();
    }
}

