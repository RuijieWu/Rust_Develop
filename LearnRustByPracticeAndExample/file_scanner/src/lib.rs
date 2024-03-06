/*
 * @Date: 2024-02-26 08:01:49
 * @LastEditTime: 2024-03-06 15:49:23
 * @Description: 本文件负责程序要用到的主要结构的声明
 * print_by_ident 函数会迭代打印整个 NodeDir 目录树 ， 它是 NodeDir 的 show 方法的实现的模块
 * NodeDir 是一个目录树 ， 它包含了保存其子文件的动态数组 ， 自身的文件信息 ， 与包含其子树的动态数组
 * FileType 是一个枚举类型 ， 用于表示文件是目录文件还是文件
 * File 是对磁盘中的文件的抽象 ， 保存了文件的文件名 ， 读写时间 ， 父目录，子目录与文件目录等信息
 * ScanResult 是用于保存扫描线程的扫描结果的结构体
 * 所有结构体均实现了 new 方法 ， 用于快速创建对应实例 ， 此外 ， NodeDir结构体还实现了 add_sub_file 和 add_sub_dir 方法 ， 用于添加对应子文件与子目录
*/
pub mod db;
pub mod scanner;
pub mod util;

use std::path::PathBuf;
use serde::{
    Serialize, 
    Deserialize
};

fn print_by_ident(n: &NodeDir, level: usize, dir_list: &mut Vec<NodeDir>) {
    let ident = "─".repeat(level);
    println!("│{} {}", ident, n.dir_info.file_name);
    let ident = " ".repeat(level);
    for i in 0..n.sub_files.len() {
        if i == n.sub_files.len()-1 {
            println!("│{} └── {}", ident, n.sub_files[i].file_name);
        }
        else {
            println!("│{} ├── {}", ident, n.sub_files[i].file_name);
        }
    }
 
    for node in &n.sub_dirs {
        dir_list.push(node.clone());
    }
}

#[derive(Debug,Clone)]
pub struct NodeDir {
    pub dir_info: File,
    pub sub_dirs: Vec<NodeDir>,
    pub sub_files: Vec<File>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum FileType {
    Directory,
    File
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct File {
    pub file_type: FileType,
    pub file_name: String,
    pub file_size: u64,
    pub modified_time: String,
    pub created_time: String,
    pub accessed_time: String,
    pub created_duration_time: u64,
    pub read_only: bool,
    pub sub_files: Vec<PathBuf>,
    pub parent_directory: PathBuf,
    pub file_path: PathBuf
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ScanResult {
    pub depth: u16,
    pub directory_number: u64,
    pub file_number: u64,
    pub longest_file_name: String
}

impl ScanResult {
    pub fn new(
        depth: u16,
        directory_number: u64,
        file_number: u64,
        longest_file_name: String
    ) -> Self {
        Self {
            depth,
            directory_number,
            file_number,
            longest_file_name
        }
    }
}

impl File {
    pub fn new(    
        file_type: FileType,
        file_name: String,
        file_size: u64,
        modified_time: String,
        created_time: String,
        accessed_time: String,
        created_duration_time: u64,
        read_only: bool,
        sub_files: Vec<PathBuf>,
        parent_directory: PathBuf,
        file_path: PathBuf
    ) -> Self {
        Self {        
            file_type,
            file_name,
            file_size,
            modified_time,
            created_time,
            accessed_time,
            created_duration_time,
            read_only,
            sub_files,
            parent_directory,
            file_path
        }
    }
}
 
impl NodeDir {
    pub fn new(dir_info: File) -> Self {
        Self {
            dir_info,
            sub_dirs:vec![],
            sub_files:vec![]
        }
    }
    pub fn add_sub_dir(&mut self,sub_dir:NodeDir) {
        self.sub_dirs.push(sub_dir);
    }
    pub fn add_sub_file(&mut self,sub_file:File) {
        self.sub_files.push(sub_file);
    }
    pub fn show(&self) {
        let mut dir_list = vec![self.clone()];
        let mut level = 0;
        while dir_list.len() > 0 {
            for dir in &dir_list.clone() {
                print_by_ident(dir, level,&mut dir_list);
                dir_list.remove(0);
            }
            level += 1;
        }
        println!("│");
    }
}
