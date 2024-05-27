mod utils;
mod error;
mod huffman;
mod node;
mod operations;
mod interface;

use interface::Args;
use operations::run;

fn main() {
    let args = Args::from_env_args().unwrap();
    match run(args) {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("{}", err.to_string());
            std::process::exit(1);
        } 
    };
}






