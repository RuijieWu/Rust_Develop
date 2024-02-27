/*
 * @Date: 2024-02-26 08:01:55
 * @LastEditTime: 2024-02-27 12:18:57
 * @Description: Handle Database Connection and store scan result
 */
use rusqlite::{Connection, Result};
use crate::{
    File,
    FilePermissions,
    FileType,
    scanner::get_file_info
};

fn init_db() -> Result<()> {
    let conn = Connection::open("file_scan.db")?;    //conn.execute(
    conn.execute(
        "create table if not exists cat_colors (
        id integer primary key,
        name text not null unique
        )",
        []
    )?;
    //let me = Person {
     //   id: 0,
      //  name: "Steven".to_string(),
       // data: None,
    //};
    //conn.execute(
     //   "INSERT INTO person (name, data) VALUES (?1, ?2)",
     //   (&me.name, &me.data),
   // )?;

 //   let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//    let person_iter = stmt.query_map([], |row| {
  //      Ok(Person {
    //        id: row.get(0)?,
      //      name: row.get(1)?,
         //   data: row.get(2)?,
        //})
    //})?;

   // for person in person_iter {
     //   println!("Found person {:?}", person.unwrap());
    //}
    Ok(())
}