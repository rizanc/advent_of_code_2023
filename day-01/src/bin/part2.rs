use utils::get_puzzle_input;

fn main() -> std::io::Result<()>{
    part2()
}

fn part2() -> std::io::Result<()>{ 
    let mut sum:u64 = 0;

    let input = get_puzzle_input(&"./input.txt")?;
    let splitted:Vec<&str>  = input.lines().collect();
    for s in splitted {

        let replaced = s.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5v")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

        let mut first:String = String::new();
        let mut last:String = String::new();
    
        for c in replaced.chars() {
            if c.is_numeric() {
                first = c.to_string();
                break;
            }

        }
        for c in replaced.chars().rev() {
            if c.is_numeric() {
                last = c.to_string();
                break;
            }
        }
        let as_string = format!("{}{}", first,last) ;
        sum += as_string.parse::<u64>().unwrap();

    }

    println!("Sum {:?}",sum);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tester() -> std::io::Result<()>{

        println!("{}",get_puzzle_input(&"./input.txt")?);
        Ok(())

    }
}