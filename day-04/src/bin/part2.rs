use std::{io::Result, collections::{BTreeSet, BTreeMap}};
use utils::get_puzzle_input;

fn main() -> Result<()> {
    part02("puzzle_input.txt")?;
    Ok(())
}

fn part02(filename: &str) -> Result<()> {
    // First column is how many number matches
    // Second column is the number of copies of this card
    let mut card_storage : BTreeMap<usize,[usize;2]> = BTreeMap::new();

    let input = get_puzzle_input(filename).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    for line in lines {
        let mut winning_storage:BTreeSet<usize> = BTreeSet::new();
        let mut matching_numbers:usize = 0;

        let mut c = line.split(":");
        let cid = c.next().unwrap().replace("Card","").trim().parse::<usize>().expect("Expected a card number");
        let mut c = c.next().unwrap().split('|'); 
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
                    matching_numbers +=1;
                }
            }
        }

        if card_storage.contains_key(&cid){
            let current = card_storage.get(&cid).unwrap();
            let inc  = current[1] +1;

            card_storage.insert(cid, [matching_numbers,inc]);

        } else {
            card_storage.insert(cid, [matching_numbers,1]);
        }
        let this_card = card_storage.get(&cid).unwrap().clone();

        for i in (cid +1) .. (cid + 1 + matching_numbers) {
            if card_storage.contains_key(&i){
                let current = card_storage.get(&i).unwrap();
                let inc  = this_card[1] + current[1];

                card_storage.insert(i, [current[0],inc]);
 
            } else {

                card_storage.insert(i, [0,(this_card[1])]);
            }
        }

    }

    let l:usize = card_storage.values().map(|v|v[1]).sum();
    dbg!(l);
    


    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        part02("puzzle_test.txt").unwrap();
    }
}
