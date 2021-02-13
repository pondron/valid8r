use valid8r::Valid8r;
use structopt::StructOpt;

// cli command: $cargo run src/main.rs --pattern main
fn main() {
    let valid = Valid8r::from_args();
    if let Err(e) = valid8r::run(valid) {
        println!("ERROR: {}", e)
    }
}
