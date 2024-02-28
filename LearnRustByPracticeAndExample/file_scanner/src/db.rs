/*
 * @Date: 2024-02-26 08:01:55
 * @LastEditTime: 2024-02-28 17:27:07
 * @Description: Handle Database Connection and store scan result
 */
use std::{
  sync::mpsc::Receiver,
  error::Error
};
use crate::{
  File,
  FilePermissions,
  FileType,
};
use rusqlite::Connection;

fn init_db() -> rusqlite::Result<Connection> {
  let conn = Connection::open("file_scan.db")?;    //conn.execute(
  conn.execute(
      "create table if not exists catalog (
      dir_name varchar(1000),
      modified_time varchar(25),
      created_time varchar(25),
      accessed_time varchar(25),
      read_only bool
      );",
      []
  )?;
  conn.execute(
    "create table if not exists file (
    file_name varchar(1000),
    catalog varchar(1000),
    file_size uint,
    modified_time varchar(25),
    created_time varchar(25),
    accessed_time varchar(25),
    read_only bool
    );",
    []
  )?;
//    foreign key (catalog)
//    references catalog(dir_name)
//    on delete cascade
  Ok(conn)
}

pub fn insert_data(conn:&Connection,file:File) -> rusqlite::Result<()> {
  if let FileType::Directory = file.file_type{
      conn.execute(
          "INSERT INTO catalog (dir_name, modified_time, created_time, accessed_time, read_only)
          VALUES (?1, ?2, ?3, ?4, ?5)",
          (
              file.file_name,
              file.modified_time,
              file.created_time,
              file.accessed_time,
              match file.file_permission{
                  FilePermissions::ReadOnly => true,
                  _ => false
              }
          )
      )?;
  }
  else if let FileType::File = file.file_type {
      conn.execute(
          "INSERT INTO file (file_name, catalog, file_size, modified_time, created_time, accessed_time, read_only)
          VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
          (
              file.file_name,
              match file.parent_directory.into_os_string().into_string(){
                  Ok(parent_directory) => parent_directory,
                  _ => String::from("None")
              },
              file.file_size,
              file.modified_time,
              file.created_time,
              file.accessed_time,
              match file.file_permission{
                  FilePermissions::ReadOnly => true,
                  _ => false
              }
          )
      )?;
  }
  Ok(())
}

pub fn db_record(file_receiver:Receiver<File>) -> Result<(),Box<dyn Error>>{
  let conn = init_db()?;
  for file in file_receiver {
      insert_data(&conn,file)?;
  }
  Ok(())
}