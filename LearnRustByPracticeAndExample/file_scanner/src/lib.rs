/*
* @Date: 2024-02-26 08:01:49
 * @LastEditTime: 2024-03-05 12:04:04
* @Description: basic functions and structures
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

pub fn find_string_difference(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    
    for (i, (char1, char2)) in str1.chars().zip(str2.chars()).enumerate() {
        if char1 != char2 {
            result.push(char1);
            result.push(char2);
            result.push_str(&str1.chars().skip(i + 1).collect::<String>());
            break;
        }
    }

    result
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
