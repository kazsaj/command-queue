mod config;
mod worker;

use std::thread;
use std::time::Duration;


fn main() {
    println!("Starting main thread");
    let handle = thread::spawn(|| worker::main());
    for i in 1..5 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    println!("Finishing main thread");
}
