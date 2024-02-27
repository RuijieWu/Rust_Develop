/*
 * @Date: 2024-02-27 09:16:10
 * @LastEditTime: 2024-02-27 18:45:12
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

pub fn parse() -> Result<PathBuf,Box<dyn Error>> {
    let args:Vec<String> = std::env::args().collect();
    let mut scan_path: PathBuf = PathBuf::new();
    if args.len() >1 && !(args[1].contains('/') || args[1].contains('\\') || args[1].contains(':')) {
        scan_path = env::current_dir()?;
    }
    scan_path.push(args[1].clone());
    Ok(scan_path)
}

pub fn record_files(file_receiver:Receiver<crate::File>) -> Result<(),Box<dyn Error>> {
    let scan_files_record = "scan_files_record";
    let scan_files = File::create(&scan_files_record)?;
    for file in file_receiver{
        if let FileType::File = file.file_type {
            to_writer(&scan_files,&file)?;
        } 
    }
    Ok(())
}

pub fn record_directories(file_receiver:Receiver<crate::File>) -> Result<(),Box<dyn Error>> {
    let scan_directories_record = "scan_directories_record_at";
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