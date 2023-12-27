mod cli;
mod db;
mod webserver;
use std::error::Error;

pub fn run() -> Result<(),Box<dyn Error>>{
    cli::parse_command()?;
    Ok(())
}