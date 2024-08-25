use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

use time::{macros::format_description, OffsetDateTime};

fn get_time() -> String {
    let st = SystemTime::now();
    let t: OffsetDateTime = st.into();
    let fd = format_description!("[hour]:[minute]:[second]");
    let time_str = t.format(fd);
    time_str.unwrap()
}

fn main() {
    let duration = Duration::from_secs(1);

    loop {
        println!("stdout: {}", get_time());
        sleep(duration);
        eprintln!("stderr: {}", get_time());
        sleep(duration);
    }
}
