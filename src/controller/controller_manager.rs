use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use vigem_client::{Client, TargetId, XGamepad, Xbox360Wired};

pub struct Foreground {
    tx: mpsc::Sender<String>,
}

struct Background {
    rx: mpsc::Receiver<String>,
    gamepad: Xbox360Wired<Client>,
}

pub struct Builder {
    fg: Foreground,
    bg: Background,
}

impl Builder {
    pub fn new() -> Self {
        // This is (kind of) your code!

        let (tx, rx) = mpsc::channel();
        let client = Client::connect().unwrap();
        let mut gamepad = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
        gamepad.plugin().unwrap();
        gamepad.wait_ready().unwrap();

        Self {
            bg: Background { rx, gamepad },
            fg: Foreground { tx },
        }
    }

    pub fn launch(self) -> Foreground {
        self.bg.run();
        self.fg
    }
}

impl Background {
    fn run(mut self) {
        // This is your code!

        let mut state = XGamepad {
            ..Default::default()
        };

        thread::spawn(move || {
            'outer: loop {
                thread::sleep(Duration::from_millis(10));

                let result = self.rx.recv().unwrap();
                if result == "exit" {
                    println!("debug exit");
                }

                let parsed = result.parse::<i16>();
                if parsed.is_ok() {
                    state.thumb_lx = parsed.unwrap();
                } else {
                    println!("parsed failed");
                    match result.as_str() {
                        "exit" => {
                            println!("exit recieved");
                            break 'outer;
                        } //break the loop and exit the thread
                        _ => println!("Recieved : {result}"),
                    }
                }

                state.right_trigger = u8::MAX;

                thread::sleep(Duration::try_from(time::Duration::milliseconds(10)).unwrap());

                let _ = self.gamepad.update(&state);

                thread::sleep(Duration::try_from(time::Duration::milliseconds(10)).unwrap());
            }

            println!("exiting the thread");
        });
    }
}

impl Foreground {
    pub fn send(&self, data: String) {
        let _ = self.tx.send(data);
    }
}
