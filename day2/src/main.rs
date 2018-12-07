use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_one() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut twos_count = 0;
    let mut threes_count = 0;
    for line in reader.lines() {
        let (twos, threes) = has_twos_and_threes(&line?);
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

fn part_two() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let lines: Vec<String> = reader.lines().map(|e| e.unwrap()).collect();
    for (i, line) in lines.iter().enumerate() {
        for other_line in lines[i + 1..].iter() {
            let same = same_chars(line, other_line);
            if same.len() == line.len() - 1 {
                println!("{}", same);
            }
        }
    }

    Ok(())
}

fn has_twos_and_threes(a: &str) -> (bool, bool) {
    let mut map = HashMap::new();
    for c in a.chars() {
        map.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    (map.values().any(|&x| x == 2), map.values().any(|&x| x == 3))
}

fn same_chars(line1: &str, line2: &str) -> String {
    line1
        .chars()
        .zip(line2.chars())
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .collect()
}
