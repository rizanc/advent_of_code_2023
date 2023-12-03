use utils::get_puzzle_input;

fn main() -> std::io::Result<()>{
    part1()
}

fn part1() -> std::io::Result<()>{ 
    let mut sum:u64 = 0;

    let input = get_puzzle_input(&"./input.txt")?;
    let splitted:Vec<&str>  = input.lines().collect();

    for s in splitted {

        println!("{:?}",s);

        let mut first:String = String::new();
        let mut last:String = String::new();
    
        for c in s.chars() {
            if c.is_numeric() {
                first = String::from(c);
                break;
            }
        }
        println!("First {}",first);
        for c in s.chars().rev() {
            if c.is_numeric() {
                last = String::from(c); //c.to_string();
                break;
            }
        }
        println!("Last {}",last);

        let as_string = format!("{}{}", first,last) ;

        println!("{}{}={}",first, last, as_string);
        
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