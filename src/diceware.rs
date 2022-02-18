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

#[cfg(test)]
mod tests {
    use crate::diceware::functions;
    use crate::diceware::CombinationMap;
    use crate::diceware::RollPhrasePair;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use tiny_die::Die;

    #[test]
    fn test_read_list_to_map() {
        let test_filename = "eff_large_wordlist.txt";
        let test_file: File = File::open(test_filename).unwrap();
        let mut test_combinations: HashMap<String, String> = HashMap::new();
        let test_reader: BufReader<File> = BufReader::new(test_file);
        for (_, test_line) in test_reader.lines().enumerate() {
            let test_line: String = test_line.unwrap();
            let mut test_iter: std::str::SplitWhitespace = test_line.split_whitespace();
            test_combinations.insert(
                String::from(test_iter.next().unwrap()),
                String::from(test_iter.next().unwrap()),
            );
        }
        assert_eq!(
            test_combinations,
            functions::read_list_to_map(test_filename)
        );
    }

    #[test]
    fn test_dice_combination() {
        let test_dee_six: Die = Die::default();
        let mut test_rolled_combo: String = String::new();
        let mut test_counter: i32 = 5;

        'test_build: loop {
            let test_dice_roll: &str = &test_dee_six.roll().to_string().to_owned()[..];
            test_rolled_combo.push_str(test_dice_roll);
            test_counter -= 1;
            if test_counter == 0 {
                break 'test_build;
            }
        }
        assert_eq!(test_rolled_combo.len(), functions::dice_combination().len());
    }

    #[test]
    fn test_make_passphrase() {
        let test_combo_map: CombinationMap = functions::read_list_to_map("eff_large_wordlist.txt");
        let mut test_passphrase: String = String::new();
        let mut test_rolls: Vec<String> = Vec::new();
        let mut test_word_count: u8 = 5;
        
        'test_build: loop {
            test_rolls.push("63432".to_string());
            test_rolls.push("32411".to_string());
            test_rolls.push("26443".to_string());
            test_rolls.push("23156".to_string());
            test_rolls.push("45236".to_string());

            for roll in &test_rolls {
                let word: & String = test_combo_map.get(roll).unwrap();
                test_passphrase += word;
                test_passphrase += " ";
                test_word_count -= 1;
                if test_word_count == 0 {
                    test_passphrase = test_passphrase.trim_end().to_string();
                    break 'test_build;
                }
            }
        }

        let test_pair: RollPhrasePair = (test_rolls, test_passphrase);
        let mut known_rolls: Vec<String> = Vec::new();
        known_rolls.push("63432".to_string());
        known_rolls.push("32411".to_string());
        known_rolls.push("26443".to_string());
        known_rolls.push("23156".to_string());
        known_rolls.push("45236".to_string());
        let known_passphrase: String = String::from("uncrown glutinous fantasize deviate premises");
        let known_pair: RollPhrasePair = (known_rolls, known_passphrase);
        assert_eq!(test_pair, known_pair);
    }
}
