use std::thread;
use std::time::Duration;

use crate::controller::controller_manager::Builder;

mod controller;

fn main() {
    let builder = Builder::new();
    let foreground = builder.launch();

    loop {
        for i in (0..=32767).step_by(1000) {
            foreground.send(i.to_string());
            thread::sleep(Duration::from_millis(20));
        }
        for i in (0..=32768).step_by(1000) {
            foreground.send((-i).to_string());
            thread::sleep(Duration::from_millis(20));
        }
    }
}
