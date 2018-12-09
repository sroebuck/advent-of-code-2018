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

    println!("Length = {}", polymer.units());

    Ok(())
}

// Module added to encapsulate implementational const with Polymer definition

mod polymer {

    const CHAR_DIFF: i8 = 'a' as i8 - 'A' as i8;

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
                if (self.sequence[i - 1] as i8 - self.sequence[i] as i8).abs() == CHAR_DIFF {
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
    }

}
