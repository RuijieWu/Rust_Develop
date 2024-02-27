/*
 * @Date: 2024-02-26 08:10:33
 * @LastEditTime: 2024-02-27 18:07:15
 * @Description: scan directory
 */
 use crate::{
    File,
    FileType,
    FilePermissions,
    ScanResult
};
use std::{
    error::Error,
    fs,
    path::PathBuf,
    sync::mpsc::{
        SyncSender,
    }
};
use chrono::prelude::*;

pub fn scan_directory(
    scan_path:PathBuf,
    scan_result:&mut ScanResult,
    scan_path_list:&mut Vec<PathBuf>,
    file_sender:&SyncSender<File>,
    directory_sender:&SyncSender<File>
) -> Result<(),Box<dyn Error>> {
    let path = match fs::metadata(&scan_path){
        Ok(ok) => ok,
        _ => return Ok(())
    };
    if path.is_file(){
        scan_result.file_number += 1;
        let file = get_file_info(scan_path)?;
        //println!("{:#?}",file);
        if file.file_name.len() > scan_result.longest_file_name.len() {
            scan_result.longest_file_name = file.file_name.clone();
        }
        file_sender.send(file)?;
        return Ok(())
    }
    let mut root_dir = get_file_info(scan_path.clone())?;
    let iterator = match fs::read_dir(&scan_path) {
        Ok(ok) => ok,
        _ => return Ok(())
    };
    for entry in iterator {
        let entry = entry?;
        let path = entry.path();
        let mut file = match get_file_info(path) {
            Ok(ok) => ok,
            _ => continue
        };
        file.parent_directory = scan_path.clone();
        root_dir.sub_files.push(PathBuf::from(&file.file_name));
        if let FileType::Directory = file.file_type  {
            scan_path_list.push(file.file_path);
        }
        else{
            //println!("{:#?}\n",file);
            scan_result.file_number += 1;
            if file.file_name.len() > scan_result.longest_file_name.len() {
                scan_result.longest_file_name = file.file_name.clone();
            }
            file_sender.send(file)?;
        }
    }
    scan_result.directory_number += 1;
    //println!("{:#?}\n",root_dir);
    if root_dir.file_name.len() > scan_result.longest_file_name.len() {
        scan_result.longest_file_name = root_dir.file_name.clone();
    }
    directory_sender.send(root_dir)?;
    Ok(())
}

pub fn get_file_info(
    file_path:PathBuf,
) -> Result<File,Box<dyn Error>> {
    let metadata = fs::metadata(&file_path)?;
    let file_type = match metadata.is_dir() {
        true => FileType::Directory,
        false => FileType::File
    };
    let file_size = metadata.len();
    let file_name = String::from(file_path.file_name().ok_or("No filename")?.to_str().ok_or("No filename")?);
    let file_permission = if metadata.permissions().readonly(){
        FilePermissions::ReadOnly
    } else {
        FilePermissions::NotReadOnly
    };
    let modified_time: DateTime<Utc> = metadata.modified()?.into();
    let accessed_time: DateTime<Utc> = metadata.accessed()?.into();
    let created_time: DateTime<Utc> = metadata.created()?.into();
    let modified_time = modified_time.format("%c").to_string();
    let accessed_time = accessed_time.format("%c").to_string();
    let created_time = created_time.format("%c").to_string();
    let file = File::new(            
        file_type,
        file_name,
        file_size,
        modified_time,
        created_time,
        accessed_time,
        file_permission,
        vec![],
        PathBuf::new(),
        file_path
    );
    Ok(file)
}