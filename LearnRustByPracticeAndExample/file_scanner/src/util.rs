/*
 * @Date: 2024-02-27 09:16:10
 * @LastEditTime: 2024-03-06 15:16:39
 * @Description: 这个文件负责程序中通用与杂项功能的实现
 * Operation 是一个枚举类型，用来表示读取到的 myfiles . txt 或 mystat . txt 中需要进行的操作，即删除，添加，修改 或 无操作
 * Command 是一个结构体，用于存储命令行参数的解析结果 , 告诉主线程与扫描线程需要执行哪些操作 ， 他实现了 new 方法，用于快速创建一个 Command 实例
 * parse 函数会获取包含命令行参数的字符串列表 ， 然后根据命令行参数生成对应的 Command 结构体
 * record_files 是开启 -yaml 选项后会开启的两个线程之一 ， 负责序列化 FileType 为 File 的 File实例 ，它会接受扫描线程发来的 File 结构体，将发来的结构体存储至文件中
 * record_directories 是开启 -yaml 选项后会开启的第二个线程 ， 负责序列化 FileType 为 Directory 的 File实例 ，它会接受扫描线程发来的 File 结构体，将发来的结构体存储至文件中
 * read_mystat 函数会读取当前目录下的 mystat.txt 文件，逐行获取扫描目录并保存到字符串数组中 ， 然后返回获取到的待扫描目录数组
 * read_myfile 函数会读取当前目录下的 myfile.txt 文件，逐行获取操作目标与对应的操作并保存到字符串数组与 Operation 数组中 ， 然后返回包含这两个数组的元组
 * ctime 函数会获取当前时间戳并以字符串的形式将其返回
 */
use std::{
    path::PathBuf,
    env,
    error::Error,
    fs::{
        File,
        read_to_string
    },
    sync::mpsc::Receiver
};
use crate::FileType;
use serde_yaml::to_writer;
use chrono::prelude::*;

#[derive(Debug,Clone)]
pub enum Operation {
    Add(u64,u64),
    Delete,
    Modify(u64,u64),
    None
}

#[derive(Debug,Clone)]
pub struct Command {
    pub scan_path       : PathBuf,
    pub db_option       : bool,
    pub yaml_option     : bool,
    pub tree_option     : bool,
    pub read_option     : bool,
    pub operation_option: bool
}

impl Command{
    fn new(
        scan_path       : PathBuf,
        db_option       : bool,
        yaml_option     : bool,
        tree_option     : bool,
        read_option     : bool,
        operation_option:bool
    ) -> Self {
        Self {
            scan_path,
            db_option,
            yaml_option,
            tree_option,
            read_option,
            operation_option
        }
    }
}

pub fn parse() -> Result<Command,Box<dyn Error>> {
    let help_message = "./file_scanner [path] [-db]/[-yaml]
    with no arguments , file_scanner will scan target path and print the files number , directories number , the longest file name and its length
    with -db argument , file_scanner will scan target path and store scan result into sqlite database created 
    with -yaml argument , file_scanner will scan target path and store scan result into two .yaml file created . One of them stored files result and the other one stored directory result
    with -tree argument , file_scanner will scan target path and build a directory tree in memory
    with -read argument , file_scanner will read mystat.txt and scan paths from it
    with -operation argument , file_scanner will read myfiles.txt and operate opertions from it";
    
    let help_message = String::from(help_message);
    let args:Vec<String> = std::env::args().collect();
    let scan_path: PathBuf = PathBuf::new();
    let mut command: Command = Command::new(scan_path,false,false,false,false,false);

    if args.len() > 1 && args.len() < 6{
        if let "-read" = args[1].as_str() {
            command.scan_path = env::current_dir()?;
            command.read_option = true;
            command.tree_option = true;
            return Ok(command)
        }
        else if let "-operation" = args[1].as_str() {
            command.scan_path = env::current_dir()?;
            command.operation_option = true;
            command.tree_option = true;
            return Ok(command)
        }
        match args[1].as_str() {
            "--help" | "-h" => {panic!("{}", help_message);}
            _ => {
                    if !(args[1].contains('/') || args[1].contains('\\') || args[1].contains(':')) {
                        command.scan_path = env::current_dir()?;
                    }
                }
        }
        command.scan_path.push(args[1].clone());
        
        if args.len() > 2 {
            match args[2].as_str() {
                "-db" => {command.db_option = true;}
                "-yaml" => {command.yaml_option = true;}
                "-tree" => {command.tree_option = true;}
                _ => {panic!("[*] wrong argument\nTry -h or --help to get more information!");}
            }
        }

        if args.len() > 3 {
            match args[3].as_str() {
                "-db" => {command.db_option = true;}
                "-yaml" => {command.yaml_option = true;}
                "-tree" => {command.tree_option = true;}
                _ => {panic!("[*] wrong argument\nTry -h or --help to get more information!");}
            }
        }

        if args.len() > 4 {
            match args[4].as_str() {
                "-db" => {command.db_option = true;}
                "-yaml" => {command.yaml_option = true;}
                "-tree" => {command.tree_option = true;}
                _ => {panic!("[*] wrong argument\nTry -h or --help to get more information!");}
            }
        }
    }
    else {
        panic!("{}", "[*] Wrong arguments number! Try -h or --help to get more information!");
    }
    Ok(command)
}

pub fn record_files(file_receiver:Receiver<crate::File>) -> Result<(),Box<dyn Error>> {
    let time = ctime()?.replace(' ',"_").replace(':',"-");
    let scan_files_record = format!["scan_files_record_at_{}.yaml",time];
    let scan_files = File::create(&scan_files_record)?;
    for file in file_receiver{
        if let FileType::File = file.file_type {
            to_writer(&scan_files,&file)?;
        } 
    }
    Ok(())
}

pub fn record_directories(file_receiver:Receiver<crate::File>) -> Result<(),Box<dyn Error>> {
    let time = ctime()?.replace(' ',"_").replace(':',"-");
    let scan_directories_record = format!["scan_directories_record_at{}.yaml",time];
    let scan_directories = File::create(&scan_directories_record)?;
    for file in file_receiver{
        if let FileType::Directory = file.file_type {
            to_writer(&scan_directories,&file)?;
        }
    }
    Ok(())
}

pub fn read_mystat() -> Result<Vec<PathBuf>,Box<dyn Error>>{
    let mut dir_list: Vec<PathBuf> = vec![];
    let content =  read_to_string("mystat.txt")?;
    for line in content.lines() {
        dir_list.push(PathBuf::from(line));
    }
    Ok(dir_list)
}

pub fn read_myfiles() -> Result<(Vec<PathBuf>,Vec<Operation>),Box<dyn Error>> {
    let mut dir_list: Vec<PathBuf> = vec![];
    let mut operation_list: Vec<Operation> = vec![];
    let content = read_to_string("myfiles.txt")?;
    for line in content.lines() {
        let mut path = line.to_string();
        let mut operation = Operation::None;
        if line.contains(',') {
            let mut time:u64 = 0;
            let mut size:u64 = 0;    
            let mut count = 1;
            for iter in line.to_string().split(',') {
                match count {
                    1 => {path = iter.to_string();}
                    2 => {
                        match iter {
                            "D" => {operation = Operation::Delete;break;},
                            "A" => {operation = Operation::Add(0,0);},
                            "M" => {operation = Operation::Modify(0,0);},
                            _ => {break;}
                        };
                    }
                    3 => {time = iter.parse::<u64>()?;}
                    4 => {size = iter.parse::<u64>()?;}
                    _ => {break;}
                }
                count += 1;
            }
            if let Operation::Add(_,_) = operation {
                operation = Operation::Add(time,size);
            }
            if let Operation::Modify(_,_) = operation {
                operation = Operation::Modify(time,size);
            }
        }
        operation_list.push(operation);
        dir_list.push(PathBuf::from(path));
    }
    Ok((dir_list,operation_list))
}

pub fn ctime() -> Result<String,Box<dyn Error>> {
    let system_time = std::time::SystemTime::now();
    let date_time: DateTime<Utc> = system_time.into();
    let ctime = date_time.format("%c").to_string();
    Ok(ctime)
}
