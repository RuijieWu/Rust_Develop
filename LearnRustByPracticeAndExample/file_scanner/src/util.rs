/*
 * @Date: 2024-02-27 09:16:10
 * @LastEditTime: 2024-02-28 17:26:55
 * @Description: parse command to choose job
 */
use std::{
    path::PathBuf,
    env,
    error::Error,
    fs::{
        File
    },
    sync::mpsc::{
        Receiver
    }
};
use crate::{
    FileType
};
use serde_yaml::to_writer;
use chrono::prelude::*;

#[derive(Debug,Clone)]
pub struct Command {
    pub scan_path   : PathBuf,
    pub db_option   : bool,
    pub yaml_option : bool,
}

impl Command{
    fn new(
        scan_path   : PathBuf,
        db_option   : bool,
        yaml_option : bool,    
    ) -> Command {
        Command {
            scan_path,
            db_option,
            yaml_option
        }
    }
}
pub fn parse() -> Result<Command,Box<dyn Error>> {
    let help_message = String::from("");
    let args:Vec<String> = std::env::args().collect();
    let scan_path: PathBuf = PathBuf::new();
    let mut command: Command = Command::new(scan_path,false,false);
    if args.len() > 1 && args.len() < 5{
        match args[1].as_str() {
            "--help" | "-h" => {panic!("{}", help_message.to_string());}
            _ => {
                    if !(args[1].contains('/') || args[1].contains('\\') || args[1].contains(':')) {
                        command.scan_path = env::current_dir()?;
                    }
                }
        }
        if args.len() > 2 {
            match args[2].as_str() {
                "-db" => {command.db_option = true;}
                "-yaml" => {command.yaml_option = true;}
                _ => {panic!("[*] wrong argument\nShould be \'-db\' or \'-yaml\' or null");}
            }
        }
        if args.len() > 3 {
            match args[3].as_str() {
                "-db" => {command.db_option = true;}
                "-yaml" => {command.yaml_option = true;}
                _ => {panic!("[*] wrong argument\nShould be \'-db\' or \'-yaml\' or null");}
            }
        }
        command.scan_path.push(args[1].clone());
    }
    else {
        panic!("{}", "[*] Wrong arguments number!");
    }
    Ok(command)
}

pub fn record_files(file_receiver:Receiver<crate::File>) -> Result<(),Box<dyn Error>> {
    let scan_files_record = "scan_files_record.yaml";
    let scan_files = File::create(&scan_files_record)?;
    for file in file_receiver{
        if let FileType::File = file.file_type {
            to_writer(&scan_files,&file)?;
        } 
    }
    Ok(())
}

pub fn record_directories(file_receiver:Receiver<crate::File>) -> Result<(),Box<dyn Error>> {
    let scan_directories_record = "scan_directories_record.yaml";
    let scan_directories = File::create(&scan_directories_record)?;
    for file in file_receiver{
        if let FileType::Directory = file.file_type {
            to_writer(&scan_directories,&file)?;
        }
    }
    Ok(())
}
pub fn show_time() -> Result<String,Box<dyn Error>> {
    let system_time = std::time::SystemTime::now();
    let date_time: DateTime<Utc> = system_time.into();
    let ctime = date_time.format("%c").to_string();
    Ok(ctime)
}