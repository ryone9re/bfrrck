use std::{env, process};

use bfrrck::exec;

extern crate bfrrck;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Argument Failed");
    }

    if let Err(err) = exec(&args[1]) {
        eprintln!("Error occured: {:?}", err);

        process::exit(1);
    }
}
