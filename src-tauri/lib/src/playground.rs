use std::fs::File;
use std::io::prelude::*;

pub fn write_file() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello world!")?;
    Ok(())
}

pub fn read_file() -> std::io::Result<String> {
    let mut file = File::open("foo.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
