use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut count: i32 = 0;
    for line in reader.lines() {
        count += line?.parse::<i32>().unwrap();
    }
    println!("{}", count);
    Ok(())
}
