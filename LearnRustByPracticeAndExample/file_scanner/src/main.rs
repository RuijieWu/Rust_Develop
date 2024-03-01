/*
 * @Date: 2024-02-26 08:01:36
 * @LastEditTime: 2024-03-01 22:01:05
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
    thread::{
        spawn,
        JoinHandle
    },
    sync::mpsc::{
        Receiver,
        sync_channel,
        SyncSender
    }
};

fn main() -> Result<(),Box<dyn Error>> {
    println!("Scan started at {}",util::ctime()?);

    let mut scan_path_list:Vec<PathBuf> = vec![];
    let command = util::parse()?;
    let scan_command = command.clone();
    scan_path_list.push(command.scan_path.clone());

    let (file_sender,file_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let (directory_sender,directory_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let (db_file_sender,db_file_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let (node_sender,node_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);

    let mut record_file_thread: JoinHandle<()>= spawn(||{});
    let mut record_directory_thread: JoinHandle<()>= spawn(||{});
    let mut db_record_thread: JoinHandle<()>= spawn(||{});
    let mut build_tree_thread: JoinHandle<()> = spawn(||{});

    let scan_thread = spawn(move || {

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
                    &directory_sender,
                    &db_file_sender,
                    &node_sender,
                    &scan_command
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

    if command.yaml_option {
        record_file_thread = spawn(||{
            match util::record_files(file_receiver){
                Ok(ok) => ok,
                Err(e) => {println!("{}",e);}
            };
        });
        record_directory_thread = spawn(||{
            match util::record_directories(directory_receiver){
                Ok(ok) => ok,
                Err(e) => {println!("{}",e);}
            };
        });
    }
    if command.db_option {
        db_record_thread = spawn(||{
            match db::db_record(db_file_receiver) {
                Ok(ok) => ok,
                Err(e) => {println!("{}",e);}
            }
        });
        }
    if command.tree_option {
        build_tree_thread = spawn(||{
            match scanner::build_tree(node_receiver,command.scan_path) {
                Ok(ok) => ok,
                Err(e) => {println!("{}",e);}
            }
        });
    }    
    
    scan_thread.join().unwrap();
    
    if command.db_option {
        db_record_thread.join().unwrap();
    }
    if command.yaml_option {
        record_file_thread.join().unwrap();
        record_directory_thread.join().unwrap();
    }
    if command.tree_option {
        build_tree_thread.join().unwrap();
    }
    
    println!("Scan completed at {}",util::ctime()?);
    Ok(())
}
