use crate::polymer::Polymer;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut polymer = Polymer::from_str(&reader.lines().next().unwrap()?);
    polymer.react();

    let polymer_units = polymer.units();
    let mut best_units = polymer_units;
    let mut best_unit_type_to_remove = b'.';
    for c in polymer.unit_types() {
        let mut p = polymer.clone();
        p.remove_units_of_type(c);
        p.react();
        let pu = p.units();
        if pu < best_units {
            best_units = pu;
            best_unit_type_to_remove = c;
        }
    }

    println!("Length = {}", polymer_units);
    println!(
        "If you remove '{}' you get the length {}",
        best_unit_type_to_remove as char, best_units
    );

    Ok(())
}

// Module added to encapsulate implementational const with Polymer definition

mod polymer {

    use std::collections::HashSet;

    const CHAR_DIFF: u8 = b'a' - b'A';

    #[derive(Clone)]
    pub struct Polymer {
        sequence: Vec<u8>,
    }

    impl Polymer {
        pub fn from_str(s: &str) -> Polymer {
            Polymer {
                sequence: s.as_bytes().to_vec(),
            }
        }

        pub fn react(&mut self) {
            let mut i = 1;
            while i < self.sequence.len() {
                if (self.sequence[i - 1] as i8 - self.sequence[i] as i8).abs() as u8 == CHAR_DIFF {
                    self.sequence.remove(i - 1);
                    self.sequence.remove(i - 1);
                    if i > 1 {
                        i -= 1;
                    }
                } else {
                    i += 1;
                }
            }
        }

        pub fn units(&self) -> usize {
            self.sequence.len()
        }

        pub fn unit_types(&self) -> HashSet<u8> {
            self.sequence
                .iter()
                .map(|&c| if c > b'Z' { c - CHAR_DIFF } else { c })
                .collect()
        }

        pub fn remove_units_of_type(&mut self, c: u8) {
            let cchar = c as char;
            let upper_char = cchar.to_uppercase().next().unwrap() as u8;
            let lower_char = cchar.to_lowercase().next().unwrap() as u8;
            let mut i = 0;
            while i < self.sequence.len() {
                let d = self.sequence[i];
                if d == upper_char || d == lower_char {
                    self.sequence.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

}
