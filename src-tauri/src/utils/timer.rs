use std::default::Default;
use std::fmt;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Copy)]
pub struct Stopwatch {
    start_time: Option<Instant>,
    elapsed: Duration,
}

impl Default for Stopwatch {
    fn default() -> Stopwatch {
        Stopwatch {
            start_time: None,
            elapsed: Duration::from_secs(0),
        }
    }
}

impl fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}ms", self.calc_ms());
    }
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        let timer: Stopwatch = Default::default();
        return timer;
    }
    pub fn init() -> Stopwatch {
        let mut timer = Stopwatch::new();
        timer.start();
        return timer;
    }
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }
    pub fn stop(&mut self) {
        self.elapsed = self.elapsed();
        self.start_time = None;
    }
    pub fn calc_ms(&self) -> i64 {
        let time = self.elapsed();
        return (time.as_secs() * 1000 + (time.subsec_nanos() / 1_000_000) as u64) as i64;
    }
    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(s1) => {
                return s1.elapsed() + self.elapsed;
            }
            None => {
                return self.elapsed;
            }
        }
    }
}
