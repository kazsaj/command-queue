use config;
use std::thread;
use std::time::Duration;

pub fn main()
{
    // actual worker logic here

    let config_value = config::get();
    println!("Config, which is empty: {}", config_value);

    for i in 1..10 {
        println!("worker {}", i);
        thread::sleep(Duration::from_millis(1));
    }

}
