use DatabaseLab::*;
fn main() {
    let conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => panic!("[*] Database Initting Failed!\n{:?}",e),
    };
    let stu1 = Student::new("123456789","张三","男","20","计算机","是");
    let cou1 = Course::new("1234","数据库","1234","4");
    let grade1 = SC::new("123456789","1234","100");
    let stu2 = Student::new("223456789","张四","男","20","计算机","是");
    let cou2 = Course::new("12345","数据库2","1234","4");
    let grade2 = SC::new("223456789","1234","100");
    let stu3 = Student::new("323456789","张五","男","20","计算机","是");
    let cou3 = Course::new("123456","数据库3","1234","4");
    let grade3 = SC::new("323456789","1234","100");

    match add_course(&conn,&cou1){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match add_student(&conn,&stu1){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match add_course(&conn,&cou2){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match add_student(&conn,&stu2){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match add_course(&conn,&cou3){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match add_student(&conn,&stu3){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match set_grade(&conn,&grade1){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match set_grade(&conn,&grade2){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match set_grade(&conn,&grade3){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match query_student(&conn,&stu1){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match query_student(&conn,&stu2){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    match query_student(&conn,&stu3){
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}",e)
    };
    drop_db(&conn);

}