use std::{
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let duration = Duration::from_secs(3);
    loop {
        println!("stdout: {:?}", Instant::now());
        sleep(duration);
        eprintln!("stderr: {:?}", Instant::now());
        sleep(duration);
    }
}
