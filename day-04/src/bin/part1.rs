use std::{io::Result, collections::BTreeSet};
use utils::get_puzzle_input;

fn main() -> Result<()> {
    part01("puzzle_input.txt")?;
    Ok(())
}

fn part01(filename: &str) -> Result<()> {
    let input = get_puzzle_input(filename).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum:usize = 0;

    for line in lines {
        let mut winning_storage:BTreeSet<usize> = BTreeSet::new();
        let mut winning_score:usize = 0;

        let mut c = line.split(":");
        let mut c = c.nth(1).unwrap().split('|'); 
        let winning = c.next().unwrap().trim().split(" ");
        for w in winning {
            if let Ok(i) = w.parse::<usize>() {
                winning_storage.insert(i);
            }
        }

        let mine = c.next().unwrap().trim().trim_end().split(' ');
        for m in mine {
            if let Ok(i) = m.parse::<usize>() {
                if winning_storage.contains(&i) {
                    if winning_score == 0 {
                        winning_score = 1;
                    } else {
                        winning_score = winning_score * 2;
                    }
    
                }
            }
        }

        sum += winning_score;

    }

    dbg!(sum);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        part01("puzzle_test.txt").unwrap();
    }
}
