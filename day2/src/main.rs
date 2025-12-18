use std::{fs::File, io::Read};

use anyhow::anyhow;
use itertools::Itertools;
use fancy_regex::Regex;

type ID = i64;

struct IDRange {
    start: ID,
    end: ID    // inclusive
}

impl IDRange {
    fn invalid_contained_ids(&self) -> Vec<ID> {
        // Valid for Step 1
        // (self.start..=self.end).filter(
        //     |i| {
        //         let i_str = i.to_string();
        //         let (first, second) = i_str.split_at(i_str.len() / 2);
        //         return (first.len() + second.len()) % 2 == 0 && first == second;
        //     }
        // ).collect()

        // Valid for Step 2 (but feels like cheating)
        // let Ok(repeat_regex) = Regex::new(r"^(\d+)\1+$") else { return Vec::new() };

        // (self.start..=self.end).filter(
        //     |i| repeat_regex.find(i.to_string().as_str()).is_ok_and(|o| o.is_some())
        // ).collect()

        // Valid for Step 2 (self-rolled, horrifying)
        (self.start..=self.end).filter(
            |i| {
                let i_str = i.to_string();
                let i_len = i_str.clone().len();
                for factor in (1..=i_len/2).filter(|n| i_len % n == 0) {
                    let chunks = i_str.clone().chars().chunks(factor).into_iter()
                                    .map(|cs| cs.collect::<Vec<char>>())
                                    .collect::<Vec<Vec<char>>>();
                    let head = chunks[0].clone();

                    if chunks.iter().all(|c| *c == head) {
                        // println!("Invalid ID {}; chunk length {}, chunk = {:?}", i, factor, head);
                        return true;
                    }
                }

                // println!("ID {} is fine", i);
                return false;
            }
        ).collect()
    }
}

impl TryFrom<&str> for IDRange {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((start, end)) = value.split("-")
                                                .map(str::parse::<ID>)
                                                .flatten()
                                                .next_tuple() 
        else { 
            return Err(anyhow!("Unable to parse range string: {}", value));
        };

        Ok(Self { start, end })
    }
}

fn parse_ranges(range_list_str: &str) -> Result<Vec<IDRange>, anyhow::Error> {
    range_list_str.split(",")
                  .map(IDRange::try_from)
                  .collect()
}

fn main() -> Result<(), anyhow::Error> {
    let mut input_file = File::open("input.txt")?;
    let mut input_str = String::new();
    input_file.read_to_string(&mut input_str)?;

    let ranges = parse_ranges(input_str.as_str())?;

    let sum = ranges.iter().map(IDRange::invalid_contained_ids)
                            .map(|v| v.iter().sum::<ID>())
                            .into_iter()
                            .sum::<ID>();

    println!("Sum of invalid IDs: {}", sum);

    Ok(())
}
