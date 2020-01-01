use std::io::Result;
use std::sync::{Arc, Mutex};

pub fn stats_loop(silent: bool, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // todo: receive the vector of bytes
        let buffer: Vec<u8> = Vec::new(); // so we can compile
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        // todo: send vector to write loop
        let quit = quit.lock().unwrap();
        if *quit {
            break;
        }
    }
    eprintln!();
    Ok(())
}
