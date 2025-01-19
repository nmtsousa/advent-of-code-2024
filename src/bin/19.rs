use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
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
    towel_map: HashMap<String, Vec<String>>,
    impossible_towel: HashSet<String>,
    count_cache: HashMap<String, usize>,
}

impl Linen {
    fn new(towels: Vec<String>) -> Self {
        let mut towel_map = HashMap::new();
        for t in towels {
            towel_map
                .entry(t[0..1].to_owned())
                .or_insert(vec![])
                .push(t);
        }
        Self {
            towel_map,
            impossible_towel: HashSet::new(),
            count_cache: HashMap::new(),
        }
    }

    fn can_be_made(&mut self, line: &str) -> bool {
        if line.is_empty() {
            return true;
        }

        if self.impossible_towel.contains(line) {
            return false;
        }

        if let Some(towels) = self.towel_map.get_mut(&line[0..1]) {
            for towel in towels.clone().iter() {
                if line.starts_with(towel) && self.can_be_made(&line[towel.len()..]) {
                    return true;
                }
            }
        }

        self.impossible_towel.insert(line.to_owned());

        false
    }

    fn possible_combo(&mut self, line: &str) -> usize {
        if line.is_empty() {
            return 1;
        }

        if self.impossible_towel.contains(line) {
            return 0;
        }

        if let Some(c) = self.count_cache.get(line) {
            return *c;
        }

        let mut count = 0;
        if let Some(towels) = self.towel_map.get_mut(&line[0..1]) {
            for towel in towels.clone().iter() {
                if line.starts_with(towel) {
                    count += self.possible_combo(&line[towel.len()..]);
                }
            }
        }

        if count == 0 {
            self.impossible_towel.insert(line.to_owned());
        }

        self.count_cache.insert(line.to_owned(), count);

        count
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

        let mut linen = Linen::new(towels);

        lines.next().expect("Empty line");

        let mut count = 0;
        for l in lines {
            if linen.can_be_made(&l) {
                count += 1;
            }
        }

        Ok(count)
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(306, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);

        let towels = lines
            .next()
            .expect("Line with towels")
            .split(",")
            .map(|s| s.trim().to_owned())
            .collect();

        let mut linen = Linen::new(towels);

        lines.next().expect("Empty line");

        let mut count = 0;
        for l in lines {
            let partial = linen.possible_combo(&l);
            count += partial;
        }

        Ok(count)
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(604622004681855, result);

    Ok(())
}
