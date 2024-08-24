use std::{thread::sleep, time::{Duration, Instant}};

fn main() {
    loop {
        println!("stdout: {:?}", Instant::now());
        sleep(Duration::from_secs(1));
        eprintln!("stderr: {:?}", Instant::now());
        sleep(Duration::from_secs(1));
    }
}
