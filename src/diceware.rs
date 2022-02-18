// read eff_large_wordlist.txt line by line
// while reading each line, add each pair to collection/map
// dice combo: word
use std::collections::HashMap;

type CombinationMap = HashMap<String, String>;
pub type RollPhrasePair = (Vec<String>, String);

pub mod functions {
    use crate::diceware::CombinationMap;
    use crate::diceware::RollPhrasePair;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use tiny_die::Die;

    pub fn read_list_to_map(filename: &str) -> CombinationMap {
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

    pub fn dice_combination() -> String {
        let dee_six: Die = Die::default();
        let mut rolled_combo: String = String::new();
        let mut counter: i32 = 5;

        'build: loop {
            let dice_roll: &str = &dee_six.roll().to_string().to_owned()[..];
            rolled_combo.push_str(dice_roll);
            counter -= 1;
            if counter == 0 {
                break 'build;
            }
        }

        return rolled_combo;
    }

    pub fn make_passphrase(mut word_count: u8, filename: &str) -> RollPhrasePair {
        let combo_map: CombinationMap = read_list_to_map(filename);
        let mut passphrase: String = String::new();
        let mut rolls: Vec<String> = Vec::new();

        'build: loop {
            let dice_rolls: String = dice_combination();
            let word: &String = combo_map.get(&dice_rolls).unwrap();
            rolls.push(dice_rolls);
            passphrase += word;
            passphrase += " ";
            word_count -= 1;
            if word_count == 0 {
                passphrase = passphrase.trim_end().to_string();
                break 'build;
            }
        }

        return (rolls, passphrase);
    }
}
