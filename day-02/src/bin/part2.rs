use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() {
    let _ = part2();
}

fn part2() -> Result<()> {
    let mut max = HashMap::<String, usize>::new();

    max.insert("blue".to_string(), 14);
    max.insert("green".to_string(), 13);
    max.insert("red".to_string(), 12);

    let mut total: usize = 0;
    let mut total_power_sum: usize = 0;

    let input = load_input("puzzle_input.txt".to_string())?;

    for line in input.lines() {
        dbg!(line);
        let mut max_game = HashMap::<String, usize>::new();

        // Get Game Id
        let parts = line.split(":").collect::<Vec<&str>>();
        let game_id = parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .expect("Expected a number");

        for picks in parts[1].split(";") {
            let pick = picks.split(",").collect::<Vec<&str>>();
            for pick_item in pick {
                let p = pick_item.trim().split(" ").collect::<Vec<&str>>();
                let item_count = p[0].trim().parse::<usize>().unwrap();
                let item_color = p[1];

                if max_game.contains_key(item_color) {
                    if *(max_game.get(item_color).unwrap()) < item_count {
                        max_game.insert(item_color.to_string(), item_count);
                    }
                } else {
                    max_game.insert(item_color.to_string(), item_count);
                }
            }
        }

        total += game_id;

        dbg!(&max_game);
        let mut game_power: usize = 0;
        for v in max_game.values() {
            if game_power == 0 {
                game_power = *v;
            } else {
                game_power = game_power * (*v);
            }
        }

        total_power_sum += game_power;
    }

    dbg!(total, total_power_sum);

    Ok(())
}

fn load_input(filename: String) -> Result<String> {
    let mut f = File::open(filename).unwrap();
    let mut buff = String::new();

    f.read_to_string(&mut buff);

    Ok(buff)
}
