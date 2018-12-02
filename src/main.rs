pub mod day1;

use std::env;

fn main() {

    let args: Vec<_> = env::args().collect();
        if args.len() > 1 {

            match args[1].as_str()
            {
                "day1" => day1::run(),
                _ => println!("Unknown day"),
            }
        }



}