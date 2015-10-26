extern crate radix_trie;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use radix_trie::Trie;


fn print_words() -> Result<(), std::io::Error> {
    let f = try!(File::open("./words"));
    let reader = BufReader::new(f);
    let mut trie: Trie<String, ()> = Trie::new();

    for line in reader.lines() {
        let line: String = try!(line).chars().rev().collect();
        let actual_line: String = line.chars().rev().collect();
        println!("{}", line);
        trie.insert(actual_line, ());
    }

    Ok(())
}

fn main() {
    match print_words() {
        Ok(_) => std::process::exit(0),
        Err(str) => println!("{}", str)
    }
}
