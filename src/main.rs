use valid8r::{Valid8r, Config};
use structopt::StructOpt;

// cli command: valid8r src/main.rs --pattern main
fn main() {
    let cfg = Config::from_args();
    let valid = Valid8r::new(cfg);

    if let Err(e) = valid.run() {
        println!("ERROR: {}", e)
    }
}
