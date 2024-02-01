use std::thread;
use std::time::Duration;

use crate::controller::controller_manager::Builder;

mod controller;

fn main() {
    // let client = vigem_client::Client::connect().unwrap();
    //
    // // Create the virtual controller target
    // let id = vigem_client::TargetId::XBOX360_WIRED;
    // let mut target = vigem_client::Xbox360Wired::new(client, id);
    //
    // // Plugin the virtual controller
    // target.plugin().unwrap();
    //
    // // Wait for the virtual controller to be ready to accept updates
    // target.wait_ready().unwrap();
    //
    // // The input state of the virtual controller
    // let mut gamepad = vigem_client::XGamepad {
    //     buttons: vigem_client::XButtons!(UP | RIGHT | LB | A | X),
    //     ..Default::default()
    // };
    //
    // let start = time::Instant::now();
    // loop {
    //     let elapsed = start.elapsed().as_seconds_f32();
    //
    //     // Play for 10 seconds
    //     if elapsed >= 10.0 {
    //         break;
    //     }
    //
    //     thread::sleep(Duration::try_from(time::Duration::milliseconds(10)).unwrap());
    //
    //
    //     let _ = target.update(&gamepad);
    //
    //     thread::sleep(Duration::try_from(time::Duration::milliseconds(10)).unwrap());
    // }

    let builder = Builder::new();
    let foreground = builder.launch();

    let mut counter = 0;
    loop {
        for i in (0..=32767).step_by(1000) {
            foreground.send(i.to_string());
            thread::sleep(Duration::from_millis(20));
        }
        for i in (0..=32768).step_by(1000) {
            foreground.send((-i).to_string());
            thread::sleep(Duration::from_millis(20));
        }

        if counter == 10 {
            println!("exited");
            thread::sleep(Duration::from_secs(100));
            foreground.send("exit".to_string());
            thread::sleep(Duration::from_secs(1));
        }

        thread::sleep(Duration::from_millis(100));
        foreground.send("banane".to_string());

        println!("{counter}");
        counter += 1;
    }
}
