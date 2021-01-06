// read a given file and return it's content

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let text = read_file(String::from("index.md"));
    println!("{}", text);
}

fn read_file(file_path: String) -> String {
    let s: String = file_path.to_owned();
    let s_slice: &str = &s[..];

    // Create a path to the desired file
    let path = Path::new(s_slice);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file. {}:", why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read file. {}", why),
        Ok(_) => {},
    }
    // `file` goes out of scope, and the "index.md" file gets closed

    String::from(s)
}
