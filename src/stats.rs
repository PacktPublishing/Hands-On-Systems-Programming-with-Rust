use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::io::{self, Result, Stderr, Write};
use std::time::{Duration, Instant};

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    let mut stderr = io::stderr();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second,
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

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes)).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{:.0}b/s]", rate)).with(Color::Blue);
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = stderr.flush();
}

trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
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
