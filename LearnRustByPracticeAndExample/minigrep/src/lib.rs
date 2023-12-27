//! lib.rs
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents ;
    if config.mode == 't' {
        contents = config.file_path;
    }
    else if config.mode == 'f' {
        contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    }
    else {
        println!("Wrong mode arguments!\nTry t for grep text OR f for grep file contents");
        return Ok(());
    }
    println!("With text:\n{contents}");
    println!("{:?}",search(&config.query,&contents));
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub mode: char
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mode: char;
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        else if args.len() == 3 {
            mode = 't';
        }
        else if args.len() == 4 {
            if args[3].len() == 1{
                mode = args[3].clone().pop().unwrap();
            }
            else {
                return Err("wrong mode argument");
            }
        }
        else{
            return Err("too many arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path, mode })
    }
}