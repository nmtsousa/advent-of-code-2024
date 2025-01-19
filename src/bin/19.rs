use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

#[derive(Debug)]
struct Linen {
    towels: Vec<String>,
}

impl Linen {
    fn new(towels: Vec<String>) -> Self {
        Self { towels }
    }

    fn can_be_made(&self, line: &str) -> bool {
        if line.is_empty() {
            return true;
        }

        for towel in self.towels.iter() {
            if line.starts_with(towel) && self.can_be_made(&line[towel.len()..]) {
                return true;
            }
        }
        false
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);

        let towels = lines
            .next()
            .expect("Line with towels")
            .split(",")
            .map(|s| s.trim().to_owned())
            .collect();

        let linen = Linen::new(towels);

        lines.next().expect("Empty line");

        let mut count = 0;
        for l in lines {
            println!("{l}");
            if linen.can_be_made(&l) {
                count += 1;
            }
        }

        println!("End");

        Ok(count)
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
