use std::collections::HashSet;
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

    let mut count: i32 = 0;
    for line in reader.lines() {
        count += line?.parse::<i32>().unwrap();
    }
    println!("{}", count);
    Ok(())
}

fn part_two() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let numbers: Vec<i32> = reader
        .lines()
        .map(|e| e.unwrap().parse::<i32>().unwrap())
        .collect();
    let mut freq_reached_set = HashSet::<i32>::new();
    freq_reached_set.insert(0);
    let mut freq = 0;
    for i in (0..numbers.len()).cycle() {
        freq += numbers[i];
        if freq_reached_set.contains(&freq) {
            break;
        }
        freq_reached_set.insert(freq);
    }

    println!("{}", freq);
    Ok(())
}
