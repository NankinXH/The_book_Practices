use std::env;
use minigrep::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(&args);
    if let Err(e) =  minigrep::run(config){
        println!("Application error: {e}");
        process::exit(1)
    }
}

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path: String = args[2].clone();
    Config { query, file_path }
  }


