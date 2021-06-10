use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use tiny_die::Die;

// read eff_large_wordlist.txt line by line
// while reading each line, add each pair to collection/map
// dice combo: word

type CombinationMap = HashMap<String, String>;
type RollPhrasePair = (Vec<String>, String);

fn read_list_to_map(filename: &str) -> CombinationMap {
    let file = File::open(filename).unwrap();
    let mut combinations: HashMap<String, String> = HashMap::new();
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line: String = line.unwrap();
        let mut iter = line.split_whitespace();
        combinations.insert(
            String::from(iter.next().unwrap()),
            String::from(iter.next().unwrap()),
        );
    }
    return combinations;
}

fn dice_combination() -> String {
    let dee_six: Die = Die::default();
    let mut rolled_combo: String = String::new();
    let mut counter = 5;

    loop {
        let dice_roll: &str = &dee_six.roll().to_string().to_owned()[..];
        rolled_combo.push_str(dice_roll);
        counter -= 1;
        if counter == 0 {
            break;
        }
    }

    return rolled_combo;
}

fn make_passphrase() -> RollPhrasePair {
    let combo_map: CombinationMap = read_list_to_map("eff_large_wordlist.txt");
    let mut passphrase: String = String::new();
    let mut rolls: Vec<String> = Vec::new();
    let mut word_count: u8 = 5; // actual word count minus 1 because iteration

    loop {
        let dice_rolls: String = dice_combination();
        let word = combo_map.get(&dice_rolls).unwrap();
        rolls.push(dice_rolls);
        passphrase += word;
        passphrase += " ";
        word_count -= 1;
        if word_count == 0 {
            break;
        }
    }

    return (rolls, passphrase);
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let roll_phrase: RollPhrasePair = make_passphrase();
        println!("\nYour passphrase:");
        println!("{}\n", roll_phrase.1);

        println!("Your rolls:");
        for roll in roll_phrase.0 {
            println!("{}", roll);
        }
    } else {
        let mut passphrase_count: u8 = args[1].parse::<u8>().unwrap();
        loop {
            let roll_phrase: RollPhrasePair = make_passphrase();
            println!("\nYour passphrase:");
            println!("{}\n", roll_phrase.1);

            println!("Your rolls:");
            for roll in roll_phrase.0 {
                println!("{}", roll);
            }
            passphrase_count -= 1;
            if passphrase_count == 0 {
                break;
            }
        }
    }

    println!("\nTime to generate: {:?}", Instant::now() - start);
}
