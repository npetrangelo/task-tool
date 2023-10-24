use std::fs::File;
use std::io::prelude::*;

pub fn write_file() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello world!")?;
    Ok(())
}

pub fn read_file() -> std::io::Result<File> {
    File::open("foo.txt")
}
