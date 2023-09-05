use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];

    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let content: String =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text: \n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path: String = args[2].clone();
    Config { query, file_path }
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
