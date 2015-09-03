extern crate langy;

use std::env;

fn main() {
    let mut arguments = env::args();
    arguments.next();
    match arguments.next() {
        Some(fname) => langy::interpret(read_file(fname)),
        None => println!("no file"),
    }
}

fn read_file(fname: String) -> String {
    use std::io::prelude::*;
    use std::fs::File;
    let mut f = File::open(fname).unwrap();
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    res
}
