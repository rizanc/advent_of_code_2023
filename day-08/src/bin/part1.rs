use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io::Result,
};
use utils::get_puzzle_input;

use std::cmp::*;

fn main() -> Result<()> {
    part01("puzzle_input.txt")?;
    Ok(())
}

fn part01(filename: &str) -> Result<()> {
    let mut lr: String = String::from("");
    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    let input = get_puzzle_input(filename).unwrap();
    let mut count = 0;
    for line in input.lines() {
        if count == 0 {
            lr = String::from(line);
            count += 1;
        } else {
            count += 1;
            if line == "" {
                continue;
            } else {
                let line = line.replace(" ", "");
                let line = line.replace("(", "");
                let line = line.replace(")", "");
                let mut l = line.split("=");

                let left = l.next().unwrap();
                let mut right = l.next().unwrap().split(",");
                
                let from = right.next().unwrap();
                let to = right.next().unwrap();

                map.insert(String::from(left), [String::from(from),String::from(to)]);
                

            }
        }
    }

    let mut step = 0;
    let start = map.get("AAA").unwrap();
    let mut current = start;
    let mut done:bool = false;

    while !done {

        for c in lr.chars() {
            step +=1;
            let side = match c {
                'L' => &current[0],
                'R' => &current[1],
                _ => panic!("Wrong Instruction Type")
            };

            //dbg!(c,current,side);
            if side == "ZZZ" {
                done = true;
            } else {
                current = map.get(side).unwrap();
            }
            
        }
       
    }


    dbg!(step);

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
