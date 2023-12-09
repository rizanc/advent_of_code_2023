use std::{io::Result, collections::BTreeSet};
use utils::get_puzzle_input;

fn main() -> Result<()> {
    part02("puzzle_input_part_2.txt")?;
    Ok(())
}

fn part02(filename: &str) -> Result<()> {
    
    let mut games = [[0 as usize;2];1];
    let mut counter = 0;

    let input = get_puzzle_input(filename).unwrap();
    let mut line_iter =  input.split("\r\n").into_iter();

    let l1 = line_iter.next().unwrap();
    let l1_split = l1.split(" ");

    for l in l1_split {
        if let Ok(v) = l.parse::<usize>() {
            games[counter] = [v,0];
            counter +=1;
        }
    }

    counter = 0;

    let l1 = line_iter.next().unwrap();
    let l1_split = l1.split(" ");


    for l in l1_split {

        if let Ok(v) = l.parse::<usize>() {

            games[counter][1] = v;
            counter +=1;
        }
    }



    dbg!(games);

    let mut total = 1;

    for game in games {
        dbg!(game);
        let mut start = 1;
        let mut winners = 0;

        while start < game[0] {

            let speed = start;
            let distance = speed * (game[0] - start);
            if distance > game[1] {
                winners +=1;

            }

            //dbg!(speed,distance,distance > game[1]);

            start += 1;
            
        }

        dbg!(winners);
        total = total * winners;
        // break;
        winners = 0;
    }


    dbg!(total);
    
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
