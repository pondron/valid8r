use structopt::StructOpt;
use std::error::Error;

#[derive(StructOpt)]
pub struct Valid8r {
    // The pattern to look for
    #[structopt(short, long)]
    pub pattern: String,

    // The path to the file to read
    // To say you want to use this field for the argument after -o or --output, 
    // you’d add #[structopt(short = "o", long = "output")]
    // https://docs.rs/structopt/0.3.21/structopt/
    #[structopt(parse(from_os_str))]
    pub file: std::path::PathBuf,
}

impl Valid8r {
}

pub fn run(valid: Valid8r) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(valid.file)
    .expect("could not read file");

    for line in content.lines() {
        if line.contains(&valid.pattern) {
            println!("{} yeehaw! youre good to go", '\u{2714}');
        } else {
            println!("{} wrongo!!!", '\u{2718}');
        }
    }
    Ok(())
}