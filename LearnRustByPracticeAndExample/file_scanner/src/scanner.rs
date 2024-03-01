/*
 * @Date: 2024-02-26 08:10:33
 * @LastEditTime: 2024-03-02 00:46:38
 * @Description: scan directory
 */
 use crate::{
    File,
    FileType,
    ScanResult,
    NodeDir,
    NodeFile,
    util::Command
};
use std::{
    error::Error,
    fs,
    path::PathBuf,
    sync::mpsc::{
        SyncSender,
        Receiver
    }
};
use chrono::prelude::*;

pub fn scan_directory(
    scan_path:PathBuf,
    scan_result:&mut ScanResult,
    scan_path_list:&mut Vec<PathBuf>,
    file_sender:&SyncSender<File>,
    directory_sender:&SyncSender<File>,
    db_file_sender:&SyncSender<File>,
    node_sender:&SyncSender<File>,
    command:&Command
) -> Result<(),Box<dyn Error>> {
    let path = match fs::metadata(&scan_path){
        Ok(ok) => ok,
        _ => return Ok(())
    };
    if path.is_file(){
        scan_result.file_number += 1;
        let file = get_file_info(scan_path)?;
        println!("{:#?}",file);
        if file.file_name.len() > scan_result.longest_file_name.len() {
            scan_result.longest_file_name = file.file_name.clone();
        }
        if command.tree_option {
            node_sender.send(file.clone())?;
        }
        if command.db_option{
            db_file_sender.send(file.clone())?;
        }
        if command.yaml_option {
            file_sender.send(file)?;
        }
        return Ok(())
    }
    let mut root_dir = get_file_info(scan_path.clone())?;
    let iterator = match fs::read_dir(&scan_path) {
        Ok(ok) => ok,
        Err(_e) => {//println!("{}",e);
            return Ok(())}
    };
    for entry in iterator {
        let entry = entry?;
        let path = entry.path();
        let mut file = match get_file_info(path) {
            Ok(ok) => ok,
            Err(_e) => {//println!("{}",e);
                continue}
        };
        file.parent_directory = scan_path.clone();
        root_dir.sub_files.push(PathBuf::from(&file.file_name));
        if root_dir.file_name.len() > scan_result.longest_file_name.len() {
            scan_result.longest_file_name = root_dir.file_name.clone();
        }
        if let FileType::Directory = file.file_type  {
            scan_path_list.push(file.file_path);
        }
        else{
            scan_result.file_number += 1;
            if file.file_name.len() > scan_result.longest_file_name.len() {
                scan_result.longest_file_name = file.file_name.clone();
            }
            if command.tree_option {
                node_sender.send(file.clone())?;
            }
            if command.db_option{
                db_file_sender.send(file.clone())?;
            }
            if command.yaml_option {
                file_sender.send(file)?;
            }
        }
    }
    scan_result.directory_number += 1;
    if command.tree_option{
        node_sender.send(root_dir.clone())?;
    }
    if command.db_option{
        db_file_sender.send(root_dir.clone())?;
    }
    if command.yaml_option {
        directory_sender.send(root_dir)?;
    }    

    Ok(())
}

fn get_file_info(file_path:PathBuf) -> Result<File,Box<dyn Error>> {
    let metadata = fs::metadata(&file_path)?;
    let file_type = match metadata.is_dir() {
        true => FileType::Directory,
        false => FileType::File
    };
    let file_size = metadata.len();
    let file_name = String::from(file_path.file_name().ok_or("No filename")?.to_str().ok_or("No filename")?);
    let read_only = metadata.permissions().readonly();
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
        read_only,
        vec![],
        PathBuf::new(),
        file_path
    );
    Ok(file)
}

fn find<'a>(root: &'a mut NodeDir, name: &'a String) -> Option<&'a mut NodeDir> {
    if root.dir_name == *name {
        return Some(root);
    }
    root.sub_dirs.iter_mut().find_map(|dir| find(dir, name))
}

pub fn build_tree(node_receiver: Receiver<File>,scan_path: PathBuf) -> Result<(),Box<dyn Error>>{
    let directory = scan_path.file_name().unwrap().to_str().unwrap().to_string();
    let mut root = NodeDir::new(directory.clone());
    let mut dir_list:Vec<String> = vec![directory];
    for node in node_receiver {
        //println!("{:?}",node.file_path);
            let parent_directory = match node.file_path.parent().unwrap().file_name(){
                Some(path) => path.to_str().unwrap().to_string(),
                _ => {println!("{:?}",node.file_path.parent().unwrap());"C:\\\\".to_string()}
            };
            for dir in &dir_list {
                if  *dir == parent_directory {
                    match node.file_type{
                        FileType::Directory =>{
                            (*find(&mut root,&parent_directory).unwrap()).add_sub_dir(NodeDir::new(node.file_name.clone()));
                            dir_list.push(node.file_name);
                        }
                        _ => {
                            (*find(&mut root,&parent_directory).unwrap()).add_sub_file(NodeFile::new(node.file_name.clone()))
                        }
                    }
                    break
                }
            }
    }
    root.show();
    Ok(())
}

/*
pub fn build_tree(node_receiver: Receiver<File>, scan_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let directory = scan_path.file_name().unwrap().to_str().unwrap().to_string();
    let mut root = NodeDir::new(directory.clone());
    let mut dir_list: Vec<String> = vec![directory.to_string()];

    for node in node_receiver {
        println!("{:?}",node.file_path);
        let parent_directory = node.file_path.parent().and_then(|p| p.file_name()).and_then(|f| f.to_str()).ok_or("Invalid file path")?.to_string();
        println!("{:?}",parent_directory);
        if let Some(parent_node) = find(&mut root, &parent_directory) {
            match node.file_type {
                FileType::Directory => {
                    parent_node.add_sub_dir(NodeDir::new(node.file_name.clone()));
                    dir_list.push(node.file_name);
                }
                _ => {
                    parent_node.add_sub_file(NodeFile::new(node.file_name.clone()));
                }
            }
        }
    }

    root.show();
    Ok(())
}
*/
