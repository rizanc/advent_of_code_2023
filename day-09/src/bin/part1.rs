use std::io::Result;
use utils::get_puzzle_input;
use itertools::{Itertools, Position};

fn main() -> Result<()> {
    process("puzzle_input.txt")?;
    Ok(())
}

fn process(filename: &str) -> Result<()> {
    let input = get_puzzle_input(filename).unwrap();
    let result = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut end_numbers: Vec<i64> = vec![];
            loop {
                if nums.iter().all(|num| num == &0) {
                    break;
                }
                nums = nums
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        match position {
                            Position::Last
                            | Position::Only => {
                                end_numbers.push(*right);
                            }
                            _ => {}
                        };
                        right - left
                    })
                    .collect::<Vec<i64>>();
            }

            let result = end_numbers
                .iter()
                .fold(0, |acc, num| acc + num);

            result
        })
        .sum::<i64>();

        dbg!(result.to_string());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        process("puzzle_test.txt").unwrap();
    }
}
