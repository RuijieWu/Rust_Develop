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
    if course.Cname != "_" {
        conn.execute(
            "UPDATE Courses SET Cname = ? WHERE Cno = ?", 
            &[course.Cname,course.Cno]
        )?;
    }
    if course.Cpno != "_" {
        conn.execute(
            "UPDATE Courses SET Cpno = ? WHERE Cno = ?", 
            &[course.Cpno,course.Cno]
        )?;
    }
    if course.Ccredit != "_" {
        conn.execute(
            "UPDATE Courses SET Ccredit = ? WHERE Cno = ?", 
            &[course.Ccredit,course.Cno]
        )?;
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
    println!("[*] Grade info set successful!");
    Ok(())
}

pub fn alter_student(conn:&Connection,student:&Student) -> Result<()> {
    if student.Sname != "_" {
        conn.execute(
            "UPDATE Students SET Sname = ? WHERE Sno = ?",
            &[student.Sname,student.Sno]
        )?;
    }
    if student.Ssex != "_" {
        conn.execute(
            "UPDATE Students SET Ssex = ? WHERE Sno = ?",
            &[student.Ssex,student.Sno]
        )?;
    }
    if student.Sage != "_" {
        conn.execute(
            "UPDATE Students SET Sage = ? WHERE Sno = ?",
            &[student.Sage,student.Sno]
        )?;
    }
    if student.Sdept != "_" {
        conn.execute(
            "UPDATE Students SET Sdept = ? WHERE Sno = ?",
            &[student.Sdept,student.Sno]
        )?;
    }
    if student.Scholarship != "_" {
        conn.execute(
            "UPDATE Students SET Scholarship = ? WHERE Sno = ?",
            &[student.Scholarship,student.Sno]
        )?;
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
        "select Students.Sdept, min(SC.Grade) from Students,SC where Students.Sno = SC.Sno group by Students.Sdept;"
    )?;
    let worst_iter = stmt.query_map([], |row| {
        Ok(Result {
            dept         : row.get(0)?,
            result       : row.get(1)?,
        })
    })?;
    for worst in worst_iter {
        println!("查询最差成绩结果如下:\n{:?}",worst?);
    }
    let mut stmt = conn.prepare(
        "select Students.Sdept, count(SC.Grade>=80)/count(*) as '优秀率' from Students,SC where Students.sno = SC.sno group by Students.Sdept;"
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
        "SELECT Students.Sdept, COUNT(*) AS '不及格人数' 
        FROM Students
        JOIN SC ON Students.sno = SC.sno
        WHERE SC.Grade < 60.00
        GROUP BY Students.Sdept;"
    )?;
    let fail_iter = stmt.query_map([], |row| {
        Ok(Result {
            dept         : row.get(0)?,
            result       : row.get(1)?,
        })
    })?;
    for fail in fail_iter {
        println!("查询不及格人数结果如下:\n{:?}",fail?);
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
    println!("查询结果如下:");
    for result in result_iter {
        println!("{:?}",result?);
    }
    Ok(())
}

pub fn query_student(conn:&Connection,student:&Student) -> Result<()> {
    #[derive(Debug)]
    struct GradeResulte {
        Cno:String,
        Cname:String,
        Cpno:String,
        Ccredit:f32,
        Grade:f32
    }
    let operation : String = format!(
        "select Courses.* , SC.Grade from Students,Courses,SC where Students.Sno = SC.Sno and SC.Cno = Courses.Cno and Students.Sno = '{}';",
        student.Sno
    );
    #[derive(Debug)]
    struct PersonalInfoResult {
        Sno         : String,
        Sname       : String,
        Ssex        : String,
        Sage        : String,
        Sdept       : String,
        Scholarship : String
    }
    let mut stmt = conn.prepare(&operation)?;
    let result_iter = stmt.query_map([],|row| {
        Ok(GradeResulte{
            Cno         : row.get(0)?,
            Cname       : row.get(1)?,
            Cpno        : row.get(2)?,
            Ccredit     : row.get(3)?,
            Grade       : row.get(4)?,
        })
    })?;
    println!("查询结果如下:");
    let operation : String = format!(
        "select * from Students where Students.Sno = '{}';",
        student.Sno
    );
    let mut stmt = conn.prepare(&operation)?;
    let info_iter = stmt.query_map([],|row| {
        Ok(PersonalInfoResult{
            Sno         : row.get(0)?,
            Sname       : row.get(1)?,
            Ssex        : row.get(2)?,
            Sage        : row.get(3)?,
            Sdept       : row.get(4)?,
            Scholarship : row.get(5)?    
        })
    })?;
    for info in info_iter {
        println!("{:?}",info?);
    }
    for result in result_iter {
        println!("{:?}",result?);
    }
    Ok(())
}

pub fn query_all(conn:&Connection,table:&str) -> Result<()> {
    match table{
        "Students" | "STUDENTS" | "students" => {
            println!("Students 表内容如下:");
            #[derive(Debug)]
            struct Result {
                Sno         : String,
                Sname       : String,
                Ssex        : String,
                Sage        : String,
                Sdept       : String,
                Scholarship : String
            }
            let mut stmt = conn.prepare(
                "SELECT * FROM Students;"
            )?;
            let students_iter = stmt.query_map([], |row| {
                Ok(Result {
                    Sno         : row.get(0)?,
                    Sname       : row.get(1)?,
                    Ssex        : row.get(2)?,
                    Sage        : row.get(3)?,
                    Sdept       : row.get(4)?,
                    Scholarship : row.get(5)?
                    })
            })?;
            for student in students_iter {
                println!("{:?}",student?);
            }

        }
        "Courses" | "COURSES" | "courses" => {
            println!("Courses 表内容如下:");
            #[derive(Debug)]
            struct Result {
                Cno     : String,
                Cname   : String,
                Cpno    : String,
                Ccredit : f32
            }
            let mut stmt = conn.prepare(
                "SELECT * FROM Courses;"
            )?;
            let courses_iter = stmt.query_map([], |row| {
                Ok(Result {
                    Cno           : row.get(0)?,
                    Cname         : row.get(1)?,
                    Cpno          : row.get(2)?,
                    Ccredit       : row.get(3)?,
                })
            })?;
            for course in courses_iter {
                println!("{:?}",course?);
            }

        }
        "SC" | "sc" => {
            println!("SC 表内容如下:");
            #[derive(Debug)]
            struct Result {
                Sno     : String,
                Cno     : String,
                Grade   : f32
            }
            let mut stmt = conn.prepare(
                "SELECT * FROM SC;"
            )?;
            let sc_iter = stmt.query_map([], |row| {
                Ok(Result {
                    Sno         : row.get(0)?,
                    Cno         : row.get(1)?,
                    Grade       : row.get(2)?,
                })
            })?;
            for sc in sc_iter {
                println!("{:?}",sc?);
            }
        },
        _ => panic!("Wrong Arguments!")
    };
    Ok(())
}