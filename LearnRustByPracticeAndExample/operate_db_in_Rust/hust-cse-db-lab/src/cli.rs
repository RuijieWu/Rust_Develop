use crate::db::*;
use crate::webserver::*;

use std::{env,process};
use std::error::Error;
use rocket::{routes,launch};

fn initDb(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    if args.len() != 2 {
        panic!("[*] Wrong Arguments!");
    }
    let _ = init_db()?;
    Ok(())
}
fn dropDb(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    if args.len() != 2 {
        panic!("[*] Wrong Arguments!");
    }
    let conn = init_db()?;
    let _ = drop_db(&conn)?;
    Ok(())
}
fn addStudent(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    let conn = init_db()?;
    if args.len() != 8 {
        panic!("[*] Wrong Arguments!");
    }
    let stu = Student::new(args[2].as_str(),args[3].as_str(),args[4].as_str(),args[5].as_str(),args[6].as_str(),args[7].as_str());
    add_student(&conn,&stu);
    Ok(())
}
fn addCourse(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    let conn = init_db()?;
    if args.len() != 6 {
        panic!("[*] Wrong Arguments!");
    }
    let cou = Course::new(args[2].as_str(),args[3].as_str(),args[4].as_str(),args[5].as_str());
    add_course(&conn,&cou);
    Ok(())
}
fn alterCourse(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    let conn = init_db()?;
    if args.len() != 6 {
        panic!("[*] Wrong Arguments!");
    }
    let cou = Course::new(args[2].as_str(),args[3].as_str(),args[4].as_str(),args[5].as_str());
    alter_course(&conn,&cou);
    Ok(())
}
fn alterStudent(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    let conn = init_db()?;
    if args.len() != 8 {
        panic!("[*] Wrong Arguments!");
    }
    let stu = Student::new(args[2].as_str(),args[3].as_str(),args[4].as_str(),args[5].as_str(),args[6].as_str(),args[7].as_str());
    alter_student(&conn,&stu);
    Ok(())
}
fn setGrade(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    let conn = init_db()?;
    if args.len() != 5 {
        panic!("[*] Wrong Arguments!");
    }
    let sc = SC::new(args[2].as_str(),args[3].as_str(),args[4].as_str());
    set_grade(&conn,&sc)?;
    Ok(())
}
fn queryCourse(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    let conn = init_db()?;
    if args.len() != 2 {
        panic!("[*] Wrong Arguments!");
    }
    query_course(&conn)?;
    Ok(())
}
fn queryDepartment(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    let conn = init_db()?;
    if args.len() != 2 {
        panic!("[*] Wrong Arguments!");
    }
    query_department(&conn);
    Ok(())
}
fn queryStudent(args: Vec<String>) -> Result<(),Box<dyn Error>> {
    let conn = init_db()?;
    if args.len() != 3 {
        panic!("[*] Wrong Arguments!");
    }
    let stu = Student::new(args[2].as_str(),"default","default","default","default","default");
    query_student(&conn,&stu);
    Ok(())
}
#[launch]
fn launch() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/",routes![generate_course])
    .mount("/",routes![generate_student])
    .mount("/",routes![generate_grade])
}
fn help() -> Result<(),Box<dyn Error>> {
    println!("./hust_cse_db_lab [option]");
    println!("Following Options can be use to work in corresponding methods ");
    println!("init                                                      -- just init the sqlite database without anything else");
    println!("drop                                                      -- drop all the tables in the database");
    println!("addStudent Sno Sname Ssex Sage Sdept Scholarship          -- add Student info");
    println!("addCourse Cno Cname Cpno Ccredit                          -- add Course info");
    println!("alterCourse Cno Cname Cpno Ccredit                        -- generate Course info");
    println!("alterStudent Sno Sname Ssex Sage Sdept Scholarship        -- generate Student info");
    println!("setGrade Sno Cno Grade                                    -- set Student-Course Grade info");
    println!("queryCourse                                               -- have a query by Course");
    println!("queryDepartment                                           -- have a query by Department");
    println!("queryStudent Sno                                          -- have a query by Student");
    println!("webserver                                                 -- work in WebServer mode and the web api is the same as the command above");
    Ok(())
}
pub fn parse_command() -> Result<(),Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        process::exit(1);
    }
    match args[1].as_str() {
        "-h" | "--help" | "-H" => help()?,
        "init" | "Init" | "INIT" => initDb(args)?,
        "drop" | "Drop" | "DROP" => dropDb(args)?,
        "addStudent" | "add_student"=> addStudent(args)?,
        "addCourse" | "add_course" => addCourse(args)?,
        "alterCourse" | "alter_course" => alterCourse(args)?,
        "alterStudent" | "alter_student" => alterStudent(args)?,
        "setGrade" | "set_grade" => setGrade(args)?,
        "queryCourse" | "query_course" => queryCourse(args)?,
        "queryDepartment" | "query_department" => queryDepartment(args)?,
        "queryStudent" | "query_student" => queryStudent(args)?,
        "Webserver" | "WEBSERVER" | "webserver" => {launch();}
        _ => panic!("Wrong Arguments!")
    }
    Ok(())
}
