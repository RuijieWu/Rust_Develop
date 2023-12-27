use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct Student<'a> {
    Sno         : &'a str,
    Sname       : &'a str,
    Ssex        : &'a str,
    Sage        : &'a str,
    Sdept       : &'a str,
    Scholarship : &'a str
}

#[derive(Debug)]
pub struct Course<'a> {
    Cno     : &'a str,
    Cname   : &'a str,
    Cpno    : &'a str,
    Ccredit : &'a str
}

#[derive(Debug)]
pub struct SC<'a> {
    Sno     :&'a str,
    Cno     :&'a str,
    Grade   :&'a str
}

impl<'a> Student<'a> {
    pub fn new(	
        Sno         : &'a str,
        Sname       : &'a str,
        Ssex        : &'a str,
        Sage        : &'a str,
        Sdept       : &'a str,
        Scholarship : &'a str
    ) -> Student<'a> {
        Student { 	
            Sno,
            Sname,
            Ssex,
            Sage,
            Sdept,
            Scholarship
        }
    }
}

impl<'a> Course<'a> {
    pub fn new(	
        Cno     : &'a str,
        Cname   : &'a str,
        Cpno    : &'a str,
        Ccredit : &'a str
    ) -> Course<'a> {
        Course { 
            Cno     : Cno,
            Cname   : Cname,
            Cpno    : Cpno,
            Ccredit : Ccredit
    }
    }
}

impl<'a> SC<'a> {
    pub fn new(	
        Sno     :&'a str,
        Cno     :&'a str,
        Grade   :&'a str
    ) -> SC<'a> {
        SC { 	
            Sno     :   Sno,
            Cno     :   Cno,
            Grade   :   Grade
        }
    }
}

//const DB_PATH : &str = "~/hust-cse-db-lab/db-lab.db";
const DB_PATH : &str = "./db-lab.db";

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "CREATE TABLE if not exists `Students` (
        `Sno` TEXT primary key,
        `Sname` TEXT not null unique,
        `Ssex` TEXT not null,
        `Sage` TEXT not null,
        `Sdept` TEXT not null,
        `Scholarship` TEXT not null
        );",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE if not exists `Courses` (
        `Cno` TEXT primary key,
        `Cname` TEXT not null,
        `Cpno` TEXT not null,
        `Ccredit` Real not null
        );",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE if not exists `SC` (
        `Sno` TEXT not null,
        `Cno` TEXT not null,
        `Grade` Real not null
        );",
        params![],
    )?;
    Ok(conn)
}

pub fn drop_db(conn:&Connection) -> Result<()> {
    conn.execute(
        "DROP TABLE IF EXISTS Students;",
        params![]
    )?;
    conn.execute(
        "DROP TABLE IF EXISTS Courses;",
        params![]
    )?;
    conn.execute(
        "DROP TABLE IF EXISTS SC;",
        params![]
    )?;
    println!("{}","[*] database droped successfully!");
    Ok(())
}

pub fn add_student( conn:&Connection, student:&Student)  -> Result<()> {
    conn.execute(
        "INSERT INTO Students (Sno,Sname,Ssex,Sage,Sdept,Scholarship) VALUES (? , ? , ? , ? , ? , ?);",
        &[student.Sno,student.Sname,student.Ssex,student.Sage,student.Sdept,student.Scholarship]
    )?;
    println!("{}", "[*] Student added successfully!");
    Ok(())
}

pub fn add_course(conn:&Connection,course:&Course) -> Result<()> {
    conn.execute(
        "INSERT INTO Courses (Cno,Cname,Cpno,Ccredit) VALUES (? , ? , ? , ?);",
        &[course.Cno,course.Cname,course.Cpno,course.Ccredit]
    )?;
    println!("{}", "[*] Course added successfully!");
    Ok(())
}

pub fn alter_course(conn:&Connection,course:&Course) -> Result<()> {
    let mut stmt = conn.prepare("UPDATE Courses SET ? = ?, WHERE Cno = ?")?;
    if course.Cname != "default" {
        stmt.execute(&["Cname", course.Cname,course.Cno])?;
    }
    if course.Cpno != "default" {
        stmt.execute(&["Cpno", course.Cpno,course.Cno])?;
    }
    if course.Ccredit != "default" {
        stmt.execute(&["Ccredit", course.Ccredit,course.Cno])?;
    }
    println!("[*] Coure info generated successful!");
    Ok(())
}

pub fn set_grade(conn:&Connection,sc:&SC) -> Result<()> {
    let operation: String = format!("SELECT * FROM {} WHERE {} = '{}'","Students","Students.Sno",sc.Sno);
    let mut stmt = conn.prepare(operation.as_str())?;
    struct StudentSQL {
        Sno         : String,
        Sname       : String,
        Ssex        : String,
        Sage        : String,
        Sdept       : String,
        Scholarship : String
    }
    struct CourseSQL {
        Cno     : String,
        Cname   : String,
        Cpno    : String,
        Ccredit : String
    }
    let mut student_iter = stmt.query_map([], |row| {
        Ok(StudentSQL {
            Sno         : row.get(0)?,
            Sname       : row.get(1)?,
            Ssex        : row.get(2)?,
            Sage        : row.get(3)?,
            Sdept       : row.get(4)?,
            Scholarship : row.get(5)?
        })
    })?;
    let operation: String = format!("SELECT * FROM {} WHERE {} = '{}'","Courses","Cno",sc.Cno);
    let mut stmt = conn.prepare(operation.as_str())?;
    let mut course_iter = stmt.query_map([], |row| {
        Ok(CourseSQL {
            Cno     : row.get(0)?,
            Cname   : row.get(1)?,
            Cpno    : row.get(2)?,
            Ccredit : row.get(3)?
        })
    })?;
    let stu = student_iter.next();
    let cou = course_iter.next();
    match  stu {
        None => panic!("SC Not Existed!"),
        _ => {
            match cou {
                None => panic!("SC Not Existed!"),
                _ => {
                    conn.execute(
                    "INSERT INTO SC (Sno,Cno,Grade) VALUES (? , ? , ?)",
                    &[sc.Sno,sc.Cno,sc.Grade]
                    )?;
                }
            }
        }
    }
    Ok(())
}

pub fn alter_student(conn:&Connection,student:&Student) -> Result<()> {
    let mut stmt = conn.prepare("UPDATE Students SET ? = ?, WHERE Sno = ?")?;
    if student.Sname != "default" {
        stmt.execute(&["Sname", student.Sname,student.Sno])?;
    }
    if student.Ssex != "default" {
        stmt.execute(&["Ssex", student.Ssex,student.Sno])?;
    }
    if student.Sage != "default" {
        stmt.execute(&["Sage", student.Sage,student.Sno])?;
    }
    if student.Sdept != "default" {
        stmt.execute(&["Sdept", student.Sdept,student.Sno])?;
    }
    if student.Scholarship != "default" {
        stmt.execute(&["Scholarship", student.Scholarship,student.Sno])?;
    }
    println!("[*] Student info generated successful!");
    Ok(())
}

pub fn query_department(conn:&Connection) -> Result<()> {
    #[derive(Debug)]
    struct Result {
        dept    : String,
        result  : f32
    }
    let mut stmt = conn.prepare(
        "select Students.Sdept, avg(SC.Grade) from Students,SC where Students.sno = SC.sno group by Students.Sdept;"
    )?;
    let avg_iter = stmt.query_map([], |row| {
        Ok(Result {
            dept         : row.get(0)?,
            result       : row.get(1)?,
        })
    })?;
    for avg in avg_iter {
        println!("查询平均成绩结果如下:\n{:?}",avg?);
    }
    let mut stmt = conn.prepare(
        "select Students.Sdept, max(SC.Grade) from Students,SC where Students.Sno = SC.Sno group by Students.Sdept;"
    )?;
    let best_iter = stmt.query_map([], |row| {
        Ok(Result {
            dept         : row.get(0)?,
            result       : row.get(1)?,
        })
    })?;
    for best in best_iter {
        println!("查询最好成绩结果如下:\n{:?}",best?);
    }
    let mut stmt = conn.prepare(
        "select Students.Sdept, count(SC.grade>=80)/count(*) as '优秀率' from Students,SC where Students.sno = SC.sno group by Students.Sdept;"
    )?;
    let rate_iter = stmt.query_map([], |row| {
        Ok(Result {
            dept         : row.get(0)?,
            result       : row.get(1)?,
        })
    })?;
    for rate in rate_iter {
        println!("查询优秀率结果如下:\n{:?}",rate?);
    }
    let mut stmt = conn.prepare(
        "select Students.Sdept, count(SC.grade<60) as '不及格人数' from Students,SC where Students.sno = SC.sno group by Students.Sdept;"
    )?;
    let fail_iter = stmt.query_map([], |row| {
        Ok(Result {
            dept         : row.get(0)?,
            result       : row.get(1)?,
        })
    })?;
    for fail in fail_iter {
        println!("查询不及格率结果如下:\n{:?}",fail?);
    }
    Ok(())
}

pub fn query_course(conn:&Connection) -> Result<()> {
    #[derive(Debug)]
    struct Result {
        Sdept:String,
        Sno:String,
        Cno:String,
        Sname:String,
        Cname:String,
        Grade:f32
    }
    let operation : String = format!(
        "select Sdept, SC.Sno, SC.Cno, Sname, Cname , Grade FROM SC JOIN Students AS S ON S.Sno = SC.Sno JOIN Courses AS C ON C.Cno = SC.Cno ORDER BY C.Cno, SC.Grade DESC;"
    );
    let mut stmt = conn.prepare(operation.as_str())?;
    let result_iter = stmt.query_map([], |row| {
        Ok(Result {
            Sdept       : row.get(0)?,
            Sno         : row.get(1)?,
            Cno         : row.get(2)?,
            Sname       : row.get(3)?,
            Cname       : row.get(4)?,
            Grade       : row.get(5)?,
        })
    })?;
    // println!("课程:\n{:?}的学生成绩如下所示:",course);
    println!("查询结果如下:");
    for result in result_iter {
        println!("{:?}",result?);
    }
    Ok(())
}

pub fn query_student(conn:&Connection,student:&Student) -> Result<()> {
    #[derive(Debug)]
    struct Result {
        Cno:String,
        Cname:String,
        Cpno:String,
        Ccredit:String,
        Grade:f32
    }
    let operation : String = format!(
        "select Courses.* , SC.Grade from Students,Courses,SC where Students.Sno = SC.Sno and SC.Cno = Courses.Cno and Students.Sno = '{}';",
        student.Sno
    );
    let mut stmt = conn.prepare(&operation)?;
    let result_iter = stmt.query_map([],|row| {
        Ok(Result{
            Cno         : row.get(0)?,
            Cname       : row.get(1)?,
            Cpno        : row.get(2)?,
            Ccredit     : row.get(3)?,
            Grade       : row.get(4)?,
        })
    })?;
    println!("学生:\n{:?}的课程成绩如下所示:",student);
    println!("查询结果如下:");
    for result in result_iter {
        println!("{:?}",result?);
    }
    Ok(())
}

