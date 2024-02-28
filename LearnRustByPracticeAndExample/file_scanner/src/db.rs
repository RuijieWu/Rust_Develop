/*
 * @Date: 2024-02-26 08:01:55
 * @LastEditTime: 2024-02-28 23:39:44
 * @Description: Handle Database Connection and store scan result
 */
 use std::{
    sync::mpsc::Receiver,
    error::Error
  };
  use crate::{
    File,
    FileType,
  };
  use rusqlite::{
      Connection,
      ToSql
  };
  
  fn init_db() -> rusqlite::Result<Connection> {
      let conn = Connection::open("file_scan.db")?;    //conn.execute(
      conn.execute_batch(
          "PRAGMA journal_mode = OFF;
          PRAGMA synchronous = 0;
          PRAGMA cache_size = 1000000;
          PRAGMA locking_mode = EXCLUSIVE;
          PRAGMA temp_store = MEMORY",
      )?;
          
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
  
  fn insert_data(conn:&Connection,file:File) -> rusqlite::Result<()> {
      if let FileType::Directory = file.file_type{
          conn.execute(
              "INSERT INTO catalog (dir_name, modified_time, created_time, accessed_time, read_only)
              VALUES (?1, ?2, ?3, ?4, ?5)",
              (
                  file.file_name,
                  file.modified_time,
                  file.created_time,
                  file.accessed_time,
                  file.read_only
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
                  file.read_only
              )
          )?;
      }
      Ok(())
  }
  
  fn insert_catalogs(conn:&mut Connection,catalogs:Vec<File>) -> rusqlite::Result<Vec<File>> {
      let mut insert_str = "(?, ?, ?, ?, ?),".repeat(catalogs.len());
      insert_str.pop();
      let insert_str = insert_str.as_str();
      let mut insert_catalog = format!["INSERT INTO catalog (dir_name, modified_time, created_time, accessed_time, read_only) VALUES {}",insert_str];
      insert_catalog.push(';');
      let tx = conn.transaction()?;
      {
          let mut stmt = tx.prepare_cached(insert_catalog.as_str())?;
          let mut row_values: Vec<&dyn ToSql> = Vec::new();
          for catalog in &catalogs {
              row_values.push(&catalog.file_name as &dyn ToSql);
              row_values.push(&catalog.modified_time as &dyn ToSql);
              row_values.push(&catalog.created_time as &dyn ToSql);
              row_values.push(&catalog.accessed_time as &dyn ToSql);
              row_values.push(&catalog.read_only as &dyn ToSql);
          }
          stmt.execute(&*row_values)?;
      }
      tx.commit()?;
      let vec:Vec<File> = Vec::new();
      Ok(vec)
  }
  
  fn insert_files(conn:&mut Connection,files:Vec<File>) -> rusqlite::Result<Vec<File>> {
      let mut insert_str = "(?, ?, ?, ?, ?, ?, ?),".repeat(files.len());
      insert_str.pop();
      let insert_str = insert_str.as_str();
      let mut insert_file = format!["INSERT INTO file (file_name, catalog, file_size, modified_time, created_time, accessed_time, read_only) VALUES {}",insert_str];
      insert_file.push(';');
      let tx = conn.transaction()?;
      {
          let mut stmt = tx.prepare_cached(insert_file.as_str())?;
          let mut row_values: Vec<&dyn ToSql> = Vec::new();
          let mut catalogs:Vec<String> = Vec::new();
          for i in 0..files.len(){
              catalogs.push(files[i].parent_directory.clone().into_os_string().into_string().expect("insert_file_catalog_into_string"));
          }
          let mut i = 0;
          for file in &files {
              row_values.push(&file.file_name as &dyn ToSql);
              row_values.push(&catalogs[i] as &dyn ToSql);
              row_values.push(&file.file_size as &dyn ToSql);
              row_values.push(&file.modified_time as &dyn ToSql);
              row_values.push(&file.created_time as &dyn ToSql);
              row_values.push(&file.accessed_time as &dyn ToSql);
              row_values.push(&file.read_only as &dyn ToSql);
              i += 1;
          }
          stmt.execute(&*row_values)?;
      }
      tx.commit()?;
      let vec:Vec<File> = Vec::new();
      Ok(vec)
  }
  
  pub fn db_record(file_receiver:Receiver<File>) -> Result<(),Box<dyn Error>>{
      let mut conn = init_db()?;
      let mut catalogs:Vec<File> = Vec::new();
      let mut files:Vec<File> = Vec::new();
      for file in file_receiver {
          match file.file_type {
              FileType::File => {catalogs.push(file);}
              _ => {files.push(file);}
          }
          if catalogs.len() > 999 {
              catalogs = insert_catalogs(&mut conn,catalogs)?;
          }
          if files.len() > 999 {
              files = insert_files(&mut conn,files)?;
          }
          //insert_data(&conn,file)?;
      }
      let _ = insert_catalogs(&mut conn,catalogs)?;
      let _ = insert_files(&mut conn,files)?;
      Ok(())
  }