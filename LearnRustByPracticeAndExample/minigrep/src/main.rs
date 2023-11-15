//! MiniGrep Demo
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).expect("Problem parsing arguments!");

    println!("Searching for {}\nIn File {}", config.query , config.file_path);
}

struct Config {
    query: String,
    file_path:String
}

impl Config {
    fn build(args: &[String]) -> Result<Config,&str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        return Ok(Config { query, file_path })
    }
}