use crossbeam::channel::Receiver;
use std::io::Result;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        total_bytes += num_bytes;
        if !silent {
            eprint!("\r{}", total_bytes);
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
