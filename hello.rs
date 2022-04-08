use std::io::Write;
use std::{thread, time::Duration};

fn main() {
    progress_bar(10, '*', 200);
    print!("\r");
    progress_bar(10, '#', 200);
    print!("\n");
}

fn progress_bar(width: u32, symbol: char, fr: u64) {
    for _progress in 0..width {
        print!("{}", symbol);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(fr));
    }
}