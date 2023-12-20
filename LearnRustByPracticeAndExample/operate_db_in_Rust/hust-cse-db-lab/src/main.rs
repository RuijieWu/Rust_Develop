use hust_cse_db_lab::db_operation::*;
use hust_cse_db_lab::parser::*;
use std::error::Error;
fn main() -> Result<(),Box<dyn Error>>{
    parse_command()?;
    Ok(())
}