/*
 * @Date: 2024-02-26 08:01:49
 * @LastEditTime: 2024-03-01 20:42:13
 * @Description: basic functions
 */
pub mod db;
pub mod scanner;
pub mod util;

use std::{
    path::PathBuf
};
use serde::{
    Serialize, 
    Deserialize
};

fn print_by_ident(n: &NodeDir, level: usize) {
    let mut ident = String::from("│ ");

    for _ in 1..level {
        ident.push_str("─");
    }
    println!("{}{}", ident, n.dir_name);
    for i in 0..n.sub_files.len() {
        if i == n.sub_files.len()-1 {
            println!("{}└── {}", ident.replace('─'," "), n.sub_files[i].file_name);
        }
        else {
            println!("{}├── {}", ident.replace('─'," "), n.sub_files[i].file_name);
        }
    }
    for node in &n.sub_dirs {
        print_by_ident(node, level + 1);
    }
}

#[derive(Debug,Clone)]
pub struct NodeDir {
    dir_name: String,
    sub_dirs: Vec<NodeDir>,
    sub_files: Vec<NodeFile>
}

#[derive(Debug,Clone)]
pub struct NodeFile {
    file_name: String
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
            read_only,
            sub_files,
            parent_directory,
            file_path
        }
    }
}

impl NodeDir {
    fn new(dir_name: String) -> Self {
        Self {
            dir_name,
            sub_dirs:vec![],
            sub_files:vec![]
        }
    }
    fn add_sub_dir(&mut self,sub_dir:NodeDir) {
        self.sub_dirs.push(sub_dir);
    }
    fn add_sub_file(&mut self,sub_dir:NodeFile) {
        self.sub_files.push(sub_dir);
    }
    fn show(&self) {
        print_by_ident(self, 0);
        println!("│");
    }}

impl NodeFile {
    fn new(file_name: String) -> Self {
        Self {
            file_name
        }
    }
}
