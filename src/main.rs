use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The pattern to look for
    #[structopt(short, long)]
    pattern: String,

    // The path to the file to read
    // To say you want to use this field for the argument after -o or --output, 
    // you’d add #[structopt(short = "o", long = "output")]
    // https://docs.rs/structopt/0.3.21/structopt/
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf
}

// cli command: $cargo run src/main.rs --pattern main

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.file)
    .expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{} yeehaw! youre good to go", '\u{2714}');
        } else {
            println!("{} wrongo!!!", '\u{2718}');
        }
    }
}
