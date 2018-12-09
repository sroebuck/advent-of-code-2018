use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

const CHAR_DIFF: i8 = 'a' as i8 - 'A' as i8;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut polymer: Vec<u8> = reader.lines().next().unwrap()?.as_bytes().to_vec();

    let mut i = 1;
    while i < polymer.len() {
        if (polymer[i - 1] as i8 - polymer[i] as i8).abs() == CHAR_DIFF {
            polymer.remove(i - 1);
            polymer.remove(i - 1);
            if i > 1 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    println!("Length = {}", polymer.len());

    Ok(())
}
