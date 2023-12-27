use rocket::http::{Header, ContentType};
use rocket::response::status::NotFound;
use rocket::{routes,launch,get};
use std::path::{PathBuf, Path};
use rocket::fs::{NamedFile, relative};
use crate::db::*;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}
//pub fn init_db() -> Result<Connection> {
//pub fn drop_db(conn:&Connection) -> Result<()> {
//pub fn add_student( conn:&Connection, student:&Student)  -> Result<()> {
//pub fn add_course(conn:&Connection,course:&Course) -> Result<()> {
//pub fn alter_course(conn:&Connection,course:&Course) -> Result<()> {
//pub fn alter_student(conn:&Connection,student:&Student) -> Result<()> {
//pub fn set_grade(conn:&Connection,sc:&SC) -> Result<()> {
//pub fn query_department(conn:&Connection,department:&str) -> Result<()> {
//pub fn query_course(conn:&Connection,course:&Course) -> Result<()> {
//pub fn query_student(conn:&Connection,student:&Student) -> Result<()> {

#[get("/student/<Sno>/<Sname>/<Ssex>/<Sage>/<Sdept>/<Scholarship>")]
pub fn generate_student(
    Sno:i32,
    Sname:&str,
    Ssex:i8,
    Sage:&str,
    Sdept:&str,
    Scholarship:&str
) -> &'static str {
    "Created Successful!"
}

#[get("/course/<Cno>/<Cname>/<Cpno>/<Ccredit>")]
pub fn generate_course(
    Cno:i32,
    Cname:&str,
    Cpno:i32,
    Ccredit:i8
) -> &str {
    "Created Successful!"
}

#[get("/student-course/<Sno>/<Cno>/<Grade>")]
pub fn generate_grade(
    Sno:i32,
    Cno:i32,
    Grade:i8
) -> &'static str {
    "Created Successful!"
}

#[get("/info/statics")]
pub fn create_student2() -> &'static str {
    "Created Successful!"
}

#[get("/info/ranking")]
pub fn student_ranking() -> &'static str {
    rank();
    "Ranking result is:"
}

#[get("/info/student/<Sno>")]
pub fn student_info(Sno:i32) -> &'static str {
    "Created Successful!"
}

fn rank()  {

}

fn exist_course (
    Sno:i32,
    Cno:i32,
    Grade:i8
) -> bool {
    false
}

fn exist_stu (
    Sno:i32,
    Sname:&str,
    Ssex:i8,
    Sage:&str,
    Sdept:&str,
    Scholarship:&str
) -> bool {
    false
}        


