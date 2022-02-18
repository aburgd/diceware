mod diceware;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filepath: PathBuf,
    word_count: Option<u8>,
}

fn main() {
    let start = Instant::now();
    let args: Cli = Cli::from_args();
    let filename: String = args.filepath.into_os_string().into_string().unwrap();
    let word_count: u8 = args.word_count.unwrap_or(5);
    let roll_phrase: diceware::RollPhrasePair =
        diceware::functions::make_passphrase(word_count, &filename);
    println!("\nYour passphrase:");
    println!("{}\n", roll_phrase.1);

    println!("Your rolls:");
    for roll in roll_phrase.0 {
        println!("{}", roll);
    }

    println!("\nTime to generate: {:?}", Instant::now() - start);
}
