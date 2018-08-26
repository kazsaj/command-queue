mod config;

fn main() {
    let config_value = config::get();
    println!("Config, which is empty: {}", config_value);
}
