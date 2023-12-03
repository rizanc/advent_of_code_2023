use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() {
    let _ = part1();
}

fn part1() -> Result<()> {
    let mut max = HashMap::<String,usize>::new();
    max.insert("blue".to_string(),14);
    max.insert("green".to_string(),13);
    max.insert("red".to_string(),12);

    let mut total:usize = 0;
    let input = load_input("puzzle_input.txt".to_string())?;

    for line in input.lines() {
        let mut is_possible = true;

        // Get Game Id
        let parts = line.split(":").collect::<Vec<&str>>();
        let game_id = parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .expect("Expected a number");

        for picks in parts[1].split(";") {
            if is_possible == false {
                break;
            }

            let pick = picks.split(",").collect::<Vec<&str>>();
            for pick_item in pick {
                let p = pick_item.trim().split(" ").collect::<Vec<&str>>();
                let item_count = p[0].trim().parse::<usize>().unwrap();
                let item_color = p[1];

                let max_for_color = max.get(item_color).unwrap();

                if item_count > *max_for_color {
                    dbg!("Not possible",picks);
                    is_possible = false;
                    break;
                }
            }
        }

        if is_possible == true {
            total += game_id;
        }
    }

    Ok(())
}

fn load_input(filename: String) -> Result<String> {
    let mut f = File::open(filename).unwrap();
    let mut buff = String::new();

    f.read_to_string(&mut buff);

    Ok(buff)
}
