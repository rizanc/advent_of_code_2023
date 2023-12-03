use std::io::Result;
use utils::get_puzzle_input;

fn main() -> Result<()> {
    part01("puzzle_input.txt")?;
    Ok(())
}

fn part01(filename: &str) -> Result<()> {
    let input = get_puzzle_input(filename).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: usize = 0;

    for (i, &line) in lines.iter().enumerate() {
        let mut start: isize = -1;
        let mut end = -1;

        for (col, c) in line.chars().enumerate() {
            
            if c.is_numeric() {
                if start == -1 {
                    start = col as isize;
                    
                }

                if start > -1 && end == -1 && col == line.len()-1 {
                    end = line.len() as isize;
                }
                
            } else {
                if start > -1 && end == -1 {
                    end = col as isize;
                }
            }

            if start > -1 && end > -1 {
                let t = line.to_string();
                let number = t
                    .get(start as usize..end as usize)
                    .expect("Expected number");
                let number = number.parse::<usize>().unwrap();

                let adjacent = is_adjacent(i, start as usize, end as usize, &lines);
                if adjacent {
                    sum += number;
                }

                start = -1;
                end = -1;
            }
        }
    }

    dbg!(sum);
    Ok(())
}

fn is_adjacent(line: usize, start: usize, end: usize, input: &Vec<&str>) -> bool {
    let mut found = false;
    let start_line = if line > 0 { line - 1 } else { 0 };
    let end_line = (line + 2).min(input.len());
    let start_col = if start > 0 { start - 1 } else { 0 };

    let i = input[0].len();
    let end_col = (end + 1).min(i);

    for line_idx in start_line..end_line {
        let line = input.get(line_idx).unwrap().chars().collect::<Vec<char>>();

        for ch_idx in start_col..end_col {
            let ch = line.get(ch_idx as usize).unwrap();

            if !ch.is_numeric() {
                if ch != &'.' {

                    found = true;
                    break;
                }
            }
        }
    }

    found
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        part01("puzzle_test.txt").unwrap();
    }
}
