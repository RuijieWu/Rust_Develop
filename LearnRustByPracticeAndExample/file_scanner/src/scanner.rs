/*
 * @Date: 2024-02-26 08:10:33
 * @LastEditTime: 2024-02-27 12:18:32
 * @Description: scan directory
 */
use crate::{
    File,
    FileType,
    FilePermissions
};
use std::{
    error::Error,
    fs,
    path::PathBuf,
    io::Write
};
use chrono::prelude::*;
use serde_yaml::to_writer;

pub fn scan_directory(
    scan_path:PathBuf,
    directory_number: &mut u64,
    file_number: &mut u64,
    longest_file_name:&mut String,
    scan_path_list:&mut Vec<PathBuf>,
    mut record:impl Write
) -> Result<(),Box<dyn Error>> {
    let path = match fs::metadata(&scan_path){
        Ok(ok) => ok,
        _ => return Ok(())
    };
    if path.is_file(){
        *file_number += 1;
        let file = get_file_info(scan_path)?;
        println!("{:#?}",file);
        to_writer(&mut record,&file)?;
        record.write(b"\n")?;
        if file.file_name.len() > longest_file_name.len() {
            *longest_file_name = file.file_name;
        }
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
            println!("{:#?}\n",file);
            to_writer(&mut record,&file)?;
            record.write(b"\n")?;
            *file_number += 1;
            if file.file_name.len() > longest_file_name.len() {
                *longest_file_name = file.file_name;
            }
    
        }
    }
    *directory_number += 1;
    println!("{:#?}\n",root_dir);
    to_writer(&mut record,&root_dir)?;
    record.write(b"\n")?;
    if root_dir.file_name.len() > longest_file_name.len() {
        *longest_file_name = root_dir.file_name;
    }

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