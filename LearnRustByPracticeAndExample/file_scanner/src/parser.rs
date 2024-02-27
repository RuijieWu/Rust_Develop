/*
 * @Date: 2024-02-27 09:16:10
 * @LastEditTime: 2024-02-27 12:18:48
 * @Description: parse command to choose job
 */
use chrono::prelude::*;
use std::{
    path::PathBuf,
    env,
    error,
    io::{self,Write},
    fs::OpenOptions
};

pub fn parse() -> Result<PathBuf,Box<dyn error::Error>> {
    let args:Vec<String> = std::env::args().collect();
    let mut scan_path: PathBuf = PathBuf::new();
    if args.len() >1 && !(args[1].contains('/') || args[1].contains('\\')) {
        scan_path = env::current_dir()?;
    }
    scan_path.push(args[1].clone());
    Ok(scan_path)
}

pub fn record() -> Result<impl Write,io::Error> {
    let file = OpenOptions::new().append(true).open("record.yaml")?;    
    Ok(file)
}

pub fn show_time() -> Result<(),Box<dyn error::Error>> {
    let system_time = std::time::SystemTime::now();
    let date_time: DateTime<Utc> = system_time.into();
    let ctime = date_time.format("%c").to_string();
    println!("{}",ctime);
    Ok(())
}