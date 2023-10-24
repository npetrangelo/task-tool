use playground::*;

mod playground;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_file() -> std::io::Result<()> {
        playground::write_file()?;
        playground::read_file()?;
        std::fs::remove_file("foo.txt")?;
        Ok(())
    }
}
