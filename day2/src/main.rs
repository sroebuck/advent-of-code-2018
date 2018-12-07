use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut twos_count = 0;
    let mut threes_count = 0;
    for line in reader.lines() {
        let l = line?;
        let (twos, threes) = has_twos_and_threes(&l);
        if twos {
            twos_count += 1
        };
        if threes {
            threes_count += 1
        };
    }
    println!(
        "{} x {} = {}",
        twos_count,
        threes_count,
        twos_count * threes_count
    );
    Ok(())
}

fn has_twos_and_threes(a: &str) -> (bool, bool) {
    let mut map = HashMap::new();
    for c in a.chars() {
        map.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    (map.values().any(|&x| x == 2), map.values().any(|&x| x == 3))
}
