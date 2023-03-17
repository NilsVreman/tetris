use std::{
    thread,
    time::{Duration, Instant},
};

use crate::game_util::{
    Job,
};

pub struct Worker {
    sleep_ms: u64,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// job = the job to perform periodically, sleep_dur = time in seconds to sleep
    pub fn new(job: Job, sleep_dur: f32) -> Self {
        let sleep_ms = (sleep_dur * 1_000.0) as u64;
        // Everything inside the closure "job" is executed periodically
        let thread = thread::spawn(move || loop {

            let start = Instant::now();

            job();

            thread::sleep(
                Duration::from_millis(sleep_ms)
                .checked_sub(start.elapsed())
                .unwrap()
            )
        });
        Self { sleep_ms, thread: Some(thread) }
    }

    // joins the current thread up with the calling function. If 
    pub fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
