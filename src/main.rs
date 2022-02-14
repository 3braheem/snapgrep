use snapgrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        std::process::exit(1);
    });
    if let Err(e) = snapgrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
