use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    println!("\nThe file contents:");
    println!("{}",
             read_username_from_file().expect(" ## File not exist! ## "));
}
