#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut fabric = Fabric::new(1000, 1000);
    for line in reader.lines() {
        let claim = Claim::parse(&line?);
        fabric.record_claim(&claim);
    }
    println!("Overclaimed {} square inches", fabric.overclaimed_space());

    Ok(())
}

struct Claim {
    _id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
}

impl Claim {
    fn parse(input: &str) -> Claim {
        let caps = RE.captures(input).unwrap();
        Claim {
            _id: caps.get(1).unwrap().as_str().parse().unwrap(),
            x: caps.get(2).unwrap().as_str().parse().unwrap(),
            y: caps.get(3).unwrap().as_str().parse().unwrap(),
            width: caps.get(4).unwrap().as_str().parse().unwrap(),
            height: caps.get(5).unwrap().as_str().parse().unwrap(),
        }
    }
}

struct Fabric {
    claim_space: Vec<Vec<i8>>,
}

impl Fabric {
    fn new(width: usize, height: usize) -> Fabric {
        Fabric {
            claim_space: vec![vec![0; height]; width],
        }
    }

    fn record_claim(&mut self, claim: &Claim) {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                self.claim_space[x][y] += 1;
            }
        }
    }

    fn overclaimed_space(&self) -> usize {
        self.claim_space
            .iter()
            .flat_map(|e| e.iter())
            .filter(|&e| *e > 1)
            .count()
    }
}
