use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // You could use the Error printer in the unwrap_or_else scope.
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1)
    }
}
