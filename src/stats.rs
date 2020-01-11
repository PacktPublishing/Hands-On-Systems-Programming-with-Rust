use crossbeam::channel::Receiver;
use std::io::Result;
use std::time::{Duration, Instant};

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut last_instant = Instant::now();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        let now = Instant::now();
        let rate_per_second = num_bytes as f64 / (now - last_instant).as_secs_f64();
        last_instant = now;
        total_bytes += num_bytes;
        if !silent {
            eprint!(
                "\r{} {} [{:.0}b/s]",
                total_bytes,
                start.elapsed().as_secs(),
                rate_per_second
            );
        }
        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}

struct Timer {
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    countdown: Duration,
    ready: bool,
}

impl Timer {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }
    fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
