use std::{
    collections::HashMap,
    io::Result,
};
use utils::get_puzzle_input;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a.abs() / gcd(a, b) * b.abs()
}

fn main() -> Result<()> {
    part02("puzzle_input.txt")?;
    Ok(())
}

fn part02(filename: &str) -> Result<()> {
    let mut results: Vec<usize> = Vec::new();
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

                map.insert(String::from(left), [String::from(from), String::from(to)]);
            }
        }
    }

    let mut start2: Vec<&[String; 2]> = Vec::new();

    for map_item in map.keys() {
        if map_item.chars().last().unwrap() == 'A' {
            start2.push(&map[map_item]);
        }
    }

    dbg!(&start2);

    for start in start2.iter() {
        let mut step = 0;
        let mut done: bool = false;

        let mut current: &[String; 2] = start;

        while !done {
            for c in lr.chars() {
                step += 1;

                let side = match c {
                    'L' => &current[0],
                    'R' => &current[1],
                    _ => panic!("Wrong Instruction Type"),
                };

                if side.chars().last().unwrap() == 'Z' {
                    done = true;
                    results.push(step as usize);
                    break;
                } else {
                    current = map.get(side).unwrap();
                }

            }
        }
    }

    let mut result = results[0] as i64;
    dbg!(&results);
    for &num in &results[1..] {
        result = lcm(result, num.try_into().unwrap());
    }

    dbg!(&result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        part02("puzzle_test_part_02.txt").unwrap();
    }
}
