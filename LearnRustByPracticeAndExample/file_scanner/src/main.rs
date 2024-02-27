/*
 * @Date: 2024-02-26 08:01:36
 * @LastEditTime: 2024-02-27 18:20:33
 * @Description: entrance of file scanner
 */
 use file_scanner::{
    db,
    scanner,
    util,
    File,
    ScanResult
};
use std::{
    error::Error,
    path::PathBuf,
    thread,
    sync::mpsc::{
        Receiver,
        sync_channel,
        SyncSender
    }
};

fn main() -> Result<(),Box<dyn Error>> {
    println!("{}",util::show_time()?);

    let mut scan_path_list:Vec<PathBuf> = vec![];
    scan_path_list.push(util::parse()?);
    let (file_sender,file_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let (directory_sender,directory_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let scan_thread = thread::spawn(move || {

        let mut scan_result = ScanResult::new(
            0,
            0,
            0,
            "".to_string()
        );
    
        while scan_path_list.len() > 0 {
            let iterator = scan_path_list.clone();
            for scan_path in iterator{
                match scanner::scan_directory(
                    scan_path.clone(),
                    &mut scan_result,
                    &mut scan_path_list,
                    &file_sender,
                    &directory_sender
                ){
                    Ok(ok) => ok,
                    Err(e) => {println!("{}",e);}
                };
                scan_path_list.remove(0);
            }
            scan_result.depth += 1;
        }
        println!("Directories number:\n{}",scan_result.directory_number);
        println!("Files number:\n{}",scan_result.file_number);
        println!("The longest file name:\n{}",scan_result.longest_file_name);
        println!("The length of the longest file name:\n{}",scan_result.longest_file_name.len());
    });
    let record_file_thread = thread::spawn(||{
        match util::record_files(file_receiver){
            Ok(ok) => ok,
            Err(e) => {println!("{}",e);}
        };
    });
    let record_directory_thread = thread::spawn(||{
        match util::record_directories(directory_receiver){
            Ok(ok) => ok,
            Err(e) => {println!("{}",e);}
        };
    });
    record_file_thread.join().unwrap();
    record_directory_thread.join().unwrap();
    scan_thread.join().unwrap();
    println!("{}",util::show_time()?);
    Ok(())
}
