use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("could ot remove file");
    println!("file is removed");
}
