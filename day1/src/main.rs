use std::{fmt::Display, fs::File, io::{BufRead, BufReader}};

use anyhow::anyhow;

struct Dial {
    position: i16
}

enum Rotation {
    Left(u16),
    Right(u16)
}

impl TryFrom::<&str> for Rotation {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            Err(anyhow!("Empty input string"))
        }
        else {            
            match (&value.chars().nth(0), &value[1..].parse()?) {
                (Some('R'), amt) => {
                    Ok(Self::Right(*amt))
                },
                (Some('L'), amt) => {
                    Ok(Self::Left(*amt))
                },
                _ => Err(anyhow!("Unable to parse input string {}", value))
            }
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotation::Left(amt) => write!(f, "L{}", amt),
            Rotation::Right(amt) => write!(f, "R{}", amt),
        }
    }
}

impl Dial {
    fn new() -> Self {
        Self {position: 50}
    }

    fn rotate(&mut self, rotation: Rotation, debug: bool) -> (i16, i16) {
        let prev_position = self.position;
        let mut rollovers = 0;

        let (operation, amt) = match rotation {
            Rotation::Left(a) => (i16::wrapping_sub as fn(i16, i16) -> i16, a),
            Rotation::Right(a) => (i16::wrapping_add as fn(i16, i16) -> i16, a),
        };

        for _ in 0..amt {
            // Compute
            self.position = operation(self.position, 1);
            // Wrap
            if self.position == -1 { self.position = 99; }
            else if self.position == 100 { self.position = 0}
            // Check
            if self.position == 0 { rollovers += 1; }
        }
        
        if debug {
            println!("{} -> {} -> {} ({} rollovers)", prev_position, rotation, self.position, rollovers);
        }

        return (self.position, rollovers);
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input_file = File::open("input.txt")?;
    let line_reader = BufReader::new(input_file);

    let mut dial = Dial::new();
    let mut password = 0;

    for line in line_reader.lines().flatten() {
        let Ok(rot) = Rotation::try_from(line.as_str()) else { continue };

        password += dial.rotate(rot, true).1;
    }

    println!("Password is {}", password);

    Ok(())
}
