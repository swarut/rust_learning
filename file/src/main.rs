// use std::fs;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
// use std::fs::File;

fn main() -> std::io::Result<()> {
    println!("Please specify path");
    let mut path = String::new();
    io::stdin().read_line(&mut path);
    let path = path.trim();
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        println!("{:?}", dir.path());
        visit_dirs(&dir.path(), &log);
    }
    Ok(())

}

fn log(path: &DirEntry) {
    println!("Path {:?}", path);
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}


// fn get_file_list_from_bridge_sort(filename: string) -> Vec<string> {
//     let mut input = File::open(filename);
//     let mut input_buffer = String::new();

// }
