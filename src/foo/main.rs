extern crate progress;

use progress::Progress;

use std::thread::sleep_ms;

pub fn main() {
    let mut p = Progress::default();

    p.start();

    while !p.finished() {
        p.increment();
        sleep_ms(250);
    }

    println!("");
}
