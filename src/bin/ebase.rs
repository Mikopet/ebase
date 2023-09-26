use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.contains(&String::from("-d")); // true means decoding

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from `stdin`");

        for c in line.chars() {
            if mode {
                print!("{}", ebase::decode(c));
            } else {
                print!("{}", ebase::encode(c));
            }
        }
    }

    println!()
}
