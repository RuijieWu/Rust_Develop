/*
 * @Date: 2024-02-26 08:01:36
 * @LastEditTime: 2024-02-27 12:18:39
 * @Description: entrance of file scanner
 */
use file_scanner::{
    db,
    scanner,
    parser
};
use std::{
    error::Error,
    path::PathBuf,
};

fn main() -> Result<(),Box<dyn Error>> {
    parser::show_time()?;

    let mut scan_path_list:Vec<PathBuf> = vec![];
    scan_path_list.push(parser::parse()?);
    let mut depth:u16 = 0;
    let mut directory_number: u64 = 0;
    let mut file_number: u64 = 0;
    let mut longest_file_name = String::from("");
    let mut file = parser::record()?;

    while scan_path_list.len() > 0 {
        let iterator = scan_path_list.clone();
        for scan_path in iterator{
            scanner::scan_directory(
                scan_path.clone(),
                &mut directory_number,
                &mut file_number,
                &mut longest_file_name,
                &mut scan_path_list,
                &mut file
            )?;
            scan_path_list.remove(0);
        }
        depth += 1;
    }

    println!(
        "{} directories\n{} files\nMax Depth:{}\nLongest File Name:{}\nLength:{}",
        directory_number,
        file_number,
        depth,
        longest_file_name,
        longest_file_name.len()
    );

    parser::show_time()?;
    Ok(())
}
