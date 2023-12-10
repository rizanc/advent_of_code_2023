use std::io::Result;
use utils::get_puzzle_input;
use lending_iterator::prelude::*;
use tracing::{debug, info};


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
                .rev()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let start_numbers =
                std::iter::from_fn(move || {
                    if nums.iter().all(|num| num == &0) {
                        None
                    } else {
                        let mut it = nums.windows_mut();
                        while let Some(
                            &mut [ref mut left, right],
                        ) = it.next()
                        {
                            *left = *left - right;
                        }

                        nums.pop()
                    }
                })
                .collect::<Vec<i64>>();

            debug!(?start_numbers);
            let result = start_numbers.iter().rev().fold(
                0,
                |acc, num| {
                    info!(acc, num, result = num - acc);
                    num - acc
                },
            );

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
