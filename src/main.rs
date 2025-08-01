use std::env;
use std::process;
use shaygrep::Config;

fn main() { 
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("in {}", config.file_path);

    if let Err(e) = shaygrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

}

