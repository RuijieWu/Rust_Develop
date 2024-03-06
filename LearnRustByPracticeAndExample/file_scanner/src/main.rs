/*
 * @Date: 2024-02-26 08:01:36
 * @LastEditTime: 2024-03-06 14:42:22
 * @Description: entrance of file scanner
 */
 use file_scanner::{
    db,
    scanner,
    util::{
        self,
        Operation,
        ctime
    },
    File,
    ScanResult,
    NodeDir,
    FileType
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
    },
};

fn main() -> Result<(),Box<dyn Error>> {

    let mut scan_path_list:Vec<PathBuf> = vec![];
    let command = util::parse()?;
    let scan_command = command.clone();
    scan_path_list.push(command.scan_path.clone());

    let (file_sender,file_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let (directory_sender,directory_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let (db_file_sender,db_file_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let (node_sender,node_receiver):(SyncSender<File>,Receiver<File>) = sync_channel(1024);
    let (tree_sender,tree_receiver):(SyncSender<NodeDir>,Receiver<NodeDir>) = sync_channel(1024);

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

        println!("Scan started at {}",util::ctime().unwrap());

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
            
        println!("[*] Directories number: {}",scan_result.directory_number);
        println!("[*] Files number: {}",scan_result.file_number);
        println!("[*] Directories depth: {}",scan_result.depth);
        println!("[*] The longest file name: {}",scan_result.longest_file_name);
        println!("[*] The length of the longest file name: {}",scan_result.longest_file_name.len());
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
            match scanner::build_tree(node_receiver,command.scan_path,tree_sender) {
                Ok(ok) => ok,
                Err(e) => {println!("{}",e);}
            }
        });
    }    
    
    scan_thread.join().unwrap();
    println!("Scan completed at {}",util::ctime()?);
    if command.db_option {
        db_record_thread.join().unwrap();
    }
    if command.yaml_option {
        record_file_thread.join().unwrap();
        record_directory_thread.join().unwrap();
    }
    if command.tree_option {
        build_tree_thread.join().unwrap();
        if !(command.read_option||command.operation_option) {
            tree_receiver.recv()?.show();
        }
    }
    if command.read_option {
        let mut tree = tree_receiver.recv()?;
        let mut mystat_threads: Vec<JoinHandle<()>> = vec![spawn(||{})];
        let dir_list = util::read_mystat()?;
        for dir in &dir_list {
            let node = scanner::find_dir(&mut tree,dir).unwrap().clone();
            mystat_threads.push(spawn(move ||{
                let output = scanner::get_dir_info(&node).unwrap();
                println!("{}",output);
            }));
        }
        for thread in mystat_threads {
            thread.join().unwrap();
        }
    }
    if command.operation_option {
        let mut tree = tree_receiver.recv()?;
        let mut myfiles_threads: Vec<JoinHandle<()>> = vec![spawn(||{})];
        let (dir_list,operation_list) = util::read_myfiles()?;
        for n in 0..dir_list.len() {
            let file_path = dir_list[n].clone();
            let operation = operation_list[n].clone();
            let mut parent_directory = file_path.clone();
            parent_directory.pop();
            let mut node = NodeDir::new(File::new(FileType::File,String::new(),0,String::new(),String::new(),String::new(),0,false,vec![],PathBuf::new(),PathBuf::new()));

            match operation.clone() {
                Operation::Add(_,_)| Operation::Delete | Operation::Modify(_,_) => {
                    node = scanner::find_dir(&mut tree,&file_path.parent().unwrap().to_path_buf()).unwrap().clone()},
                _ => {
                    node = scanner::find_dir(&mut tree,&file_path).expect(format!["{:?} -> {:?} Error!",operation,file_path].as_str()).clone();}
            }
            let output = scanner::get_dir_info(&node)?;
            print!("\nBefore:\n{}",output);
            println!("{:?} -> {:?}",operation,file_path);
            myfiles_threads.push(spawn(move ||{
                match operation.clone() {
                    Operation::None => {();},
                    Operation::Add(time,size) => {
                        let name = file_path.clone().file_name().unwrap().to_str().unwrap().to_string();
                        let ctime = ctime().unwrap();
                        let file = File::new(FileType::File,name,size,ctime.clone(),ctime.clone(),ctime,time,false,vec![],parent_directory,file_path);
                        node.add_sub_file(file);
                    },
                    Operation::Delete => {
                        for i in 0..node.sub_dirs.len() {
                            if node.sub_dirs[i].dir_info.file_path == file_path {
                                node.sub_dirs.remove(i);
                                break
                            }
                        }
                        for i in 0..node.sub_files.len() {
                            if node.sub_files[i].file_path == file_path {
                                node.sub_files.remove(i);
                                break
                            }
                        }
                    },
                   Operation::Modify(time,size) => {
                        for file in &mut node.sub_files {
                            if file.file_path == file_path {
                                file.file_size = size;
                                file.created_duration_time = time;
                                break
                            }
                        }
                    }
                };
            let output = scanner::get_dir_info(&node).unwrap();
            println!("After:\n{}",output);
            }));
        }
        for thread in myfiles_threads {
            thread.join().unwrap();
        }
    }

    Ok(())
}
