use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;
use tiny_die::Die;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filepath: PathBuf,
    word_count: Option<u8>,
}

// read eff_large_wordlist.txt line by line
// while reading each line, add each pair to collection/map
// dice combo: word

type CombinationMap = HashMap<String, String>;
type RollPhrasePair = (Vec<String>, String);

fn read_list_to_map(filename: &str) -> CombinationMap {
    let file: File = File::open(filename).unwrap();
    let mut combinations: HashMap<String, String> = HashMap::new();
    let reader: BufReader<File> = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line: String = line.unwrap();
        let mut iter: std::str::SplitWhitespace = line.split_whitespace();
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
    let mut counter: i32 = 5;

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

fn make_passphrase(mut word_count: u8, filename: &str) -> RollPhrasePair {
    let combo_map: CombinationMap = read_list_to_map(filename);
    let mut passphrase: String = String::new();
    let mut rolls: Vec<String> = Vec::new();

    loop {
        let dice_rolls: String = dice_combination();
        let word: &String = combo_map.get(&dice_rolls).unwrap();
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
    let args: Cli = Cli::from_args();
    let filename: String = args.filepath.into_os_string().into_string().unwrap();
    let word_count: u8 = args.word_count.unwrap_or(5);
    let roll_phrase: RollPhrasePair = make_passphrase(word_count, &filename);
    println!("\nYour passphrase:");
    println!("{}\n", roll_phrase.1);

    println!("Your rolls:");
    for roll in roll_phrase.0 {
        println!("{}", roll);
    }

    println!("\nTime to generate: {:?}", Instant::now() - start);
}
