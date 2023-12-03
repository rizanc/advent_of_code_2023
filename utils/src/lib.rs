use std::{fs::File, io::Read};

pub fn get_puzzle_input(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    return Ok(contents);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_puzzle_input("puzzle_input.txt");
        dbg!(result);

    }
}
