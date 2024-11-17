extern crate libs;
extern crate engine;

use libs::log_duration;
use engine::tokenize;

use std::fs;


use std::time::Duration;
use std::{sync, thread, time};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Timer {
    duration: Duration,
    handle: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
}

impl Timer {
    pub fn new(duration: Duration) -> Timer {
        Timer {
            duration: duration,
            handle: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start<F>(&mut self, fun: F)
        where F: 'static + Send + FnMut() -> ()
    {
        self.alive.store(true, Ordering::SeqCst);

        let alive = self.alive.clone();

        self.handle = Some(thread::spawn(move || {
            let mut fun = fun;
            while alive.load(Ordering::SeqCst) {
                fun();
                thread::sleep(self.duration.clone());
            }
        }));
    }

    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
        self.handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
    }
}



#[log_duration]
#[must_use]
fn function_to_benchmark() -> u16 {
    let mut counter = 0;
    for _ in 0..u16::MAX {
        counter += 1;
    }

    counter
}

fn main() {
    // println!("{}", function_to_benchmark());
    println!("Hello, world!");
    let input = fs::read_to_string("test/tmp/test01.py").unwrap();
    // let input = "let x = 42 + 3;";
    // let mut lexer = Lexer::new(input);
    let tokens = tokenize(&input);

    for token in tokens {
        println!("{:?}", token);
    }
    println!("{:?}", &input[0..6]);


    let mut timer = Timer::new(time::Duration::from_secs(4));
    timer.start(|| println!("Hello, World!") );

    println!("Feeling sleepy...");
    thread::sleep(time::Duration::from_millis(10000));

    println!("Time for dinner!");
    timer.stop();
    
}
