/*
 * @Date: 2024-02-26 08:10:33
 * @LastEditTime: 2024-03-16 16:56:05
 * @Description: 这个文件负责与扫描相关的操作 ， 包括扫描线程与建立目录树线程的相关函数
 * scan_directory 函数接收通道的发送器，扫描路径 ， 待扫描路径列表与扫描指令作为参数 ， 会根据扫描指令决定向哪些通道发送扫描获取到的 File 实例 ， 同时把当前路径下的新的目录的路径放入到待扫描路径中
 * get_dir_info 函数会获取一个目录结构体的不可变引用 ， 然后统计其下的最新与最久文件，总文件个数与总文件大小. (不包含其子目录)
 * get_file_info 函数会获取指定路径文件的文件信息 ， 生成保存了对那个文件信息的 File 结构体实例作为返回
 * find_dir 函数获取目录树和路径作为参数，然后通过迭代查找找到目录树中与文件路径与参数路径相同的结点的可变引用
 * build_tree 函数会接收扫描线程发来的 File 实例 ， 根据 File 实例中的信息将其插入到目录树中 ， 插入完毕后返回目录树
 */
 use crate::{
    File,
    FileType,
    ScanResult,
    NodeDir,
    util::Command
};
use std::{
    error::Error,
    fs,
    path::PathBuf,
    sync::mpsc::{
        SyncSender,
        Receiver
    },
    time::SystemTime,
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
    if command.tree_option{
        node_sender.send(root_dir.clone())?;
    }    
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
    if command.db_option{
        db_file_sender.send(root_dir.clone())?;
    }
    if command.yaml_option {
        directory_sender.send(root_dir)?;
    }  
    scan_result.directory_number += 1;  
    Ok(())
}

pub fn get_dir_info(node:&NodeDir) -> Result<String,Box<dyn Error>> {
    //file_info:(FileName,FileSize,CreatedTime)
    let mut earliest_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let mut latest_time:u64 = 0;
    let mut earliest_file = File::new(FileType::File,String::new(),0,String::new(),String::new(),String::new(),0,false,vec![],PathBuf::new(),PathBuf::new());
    let mut latest_file = File::new(FileType::File,String::new(),0,String::new(),String::new(),String::new(),0,false,vec![],PathBuf::new(),PathBuf::new());
    let total_file_number = node.sub_files.len() + node.sub_dirs.len();
    let mut total_file_size: u64 = 0;
    for file in &node.sub_files {
        if file.created_duration_time > latest_time {
            latest_file = file.clone();
            latest_time = file.created_duration_time;
        }
        if file.created_duration_time < earliest_time {
            earliest_file = file.clone();
            earliest_time = file.created_duration_time;
        }
        total_file_size += file.file_size;
    }
    let mut output = String::new();
    output.push_str(format![
        "[*] In directory {:#?}, there are {} files and the total file size is {} bytes\n",
        node.dir_info.file_path,
        total_file_number,
        total_file_size
    ].as_str());
    output.push_str(format![
        "[*] The latest file is {} , created at {} and it occupies {} bytes of space.\n",
        latest_file.file_name,
        latest_file.created_time,
        latest_file.file_size
    ].as_str());
    output.push_str(format![
        "[*] The earliest file is {} , created at {} and it occupies {} bytes of space.\n",
        earliest_file.file_name,
        earliest_file.created_time,
        earliest_file.file_size
    ].as_str());
    Ok(output)
}

pub fn get_file_info(file_path:PathBuf) -> Result<File,Box<dyn Error>> {
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
    let created_duration_time = metadata.created()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let file = File::new(            
        file_type,
        file_name,
        file_size,
        modified_time,
        created_time,
        accessed_time,
        created_duration_time,
        read_only,
        vec![],
        PathBuf::new(),
        file_path
    );
    Ok(file)
}

pub fn find_dir<'a>(root: &'a mut NodeDir, name: &'a PathBuf) -> Option<&'a mut NodeDir> {
    let mut stack = Vec::new();
    stack.push(root);

    while let Some(node) = stack.pop() {
        if node.dir_info.file_path == *name {
            return Some(node);
        }
        for sub_dir in &mut node.sub_dirs {
            stack.push(sub_dir);
        }
    }

    None
}

pub fn build_tree(
    node_receiver: Receiver<File>,
    scan_path: PathBuf,
    tree_sender: SyncSender<NodeDir>,
) -> Result<(), Box<dyn Error>> {
    let mut root = NodeDir::new(get_file_info(scan_path.clone())?);
    for node in node_receiver {
        let mut parent_directory = node.file_path.clone();
        parent_directory.pop();
        match node.file_type {
            FileType::Directory => {
                if let Some(parent_dir) = find_dir(&mut root, &parent_directory) {
                    parent_dir.add_sub_dir(NodeDir::new(node));
                } else {
                    eprintln!("[*] Directory not found: {:?}", parent_directory);
                }
            },
            _ => {
                if let Some(parent_dir) = find_dir(&mut root, &parent_directory) {
                    parent_dir.add_sub_file(node);
                } else {
                    eprintln!("[*] Directory not found: {:?}", parent_directory);
                }
            }
        };
    }
    tree_sender.send(root)?;
    Ok(())
}