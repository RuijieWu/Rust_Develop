/*
 * @Date: 2024-02-26 08:01:49
 * @LastEditTime: 2024-02-28 16:27:59
 * @Description: basic functions
 */
pub mod db;
pub mod scanner;
pub mod util;

use std::{
    error::Error,
    path::PathBuf
};
use serde::{
    Serialize, 
    Deserialize
};
//use chrono::prelude::{
//    DateTime,
//    Utc
//};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum FilePermissions {
    ReadOnly,
    NotReadOnly
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
    pub file_permission: FilePermissions,
    pub sub_files: Vec<PathBuf>,
    pub parent_directory: PathBuf,
    pub file_path: PathBuf
}

#[derive(Debug)]
pub struct TreeNode {
    pub file_info: File,
    pub kid: Option<Box<TreeNode>>,
    pub bro: Option<Box<TreeNode>>   
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
    ) -> ScanResult {
        ScanResult{
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
        file_permission: FilePermissions,
        sub_files: Vec<PathBuf>,
        parent_directory: PathBuf,
        file_path: PathBuf
        ) -> File {
        File{        
            file_type,
            file_name,
            file_size,
            modified_time,
            created_time,
            accessed_time,
            file_permission,
            sub_files,
            parent_directory,
            file_path
        }
    }
}

impl TreeNode {
    pub fn set_file_info(&mut self,file_info:File) -> Result<(),Box<dyn Error>> {
        // ! Check if file_info is legal
        // ! if let File {is_directory,filename,subfiles,parent_directory} = file_info {
        // ! }
        self.file_info = file_info;
        Ok(())
    }
    pub fn insert_kid(&mut self,node:TreeNode) -> Result<(),Box<dyn Error>> {
        self.kid = Some(Box::new(node));
        Ok(())
    }
    pub fn insert_bro(&mut self,node:TreeNode) -> Result<(),Box<dyn Error>> {
        self.bro = Some(Box::new(node));
        Ok(())
    }
    pub fn delete_kid(&mut self) -> Result<(),Box<dyn Error>> {
        self.kid = None;
        Ok(())
    }
    pub fn delete_bro(&mut self) -> Result<(),Box<dyn Error>> {
        self.bro = None;
        Ok(())
    }

}
