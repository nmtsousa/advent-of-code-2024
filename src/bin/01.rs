use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut left = vec![];
        let mut right = vec![];

        for line in reader.lines() {
            let read_line = line.unwrap();
            let mut items = read_line.split_whitespace();
            left.push(items.next().unwrap().parse::<i32>().unwrap());
            right.push(items.next().unwrap().parse::<i32>().unwrap());
        }

        left.sort();
        right.sort();

        let result: i32 = std::iter::zip(left, right)
            .map(|(l, r)| (l - r).abs())
            .sum();

        Ok(result as usize)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut left = vec![];
        let mut right = vec![];

        for line in reader.lines() {
            let read_line = line.unwrap();
            let mut items = read_line.split_whitespace();
            left.push(items.next().unwrap().parse::<i32>().unwrap());
            right.push(items.next().unwrap().parse::<i32>().unwrap());
        }

        let result: i32 = left
            .iter()
            .map(|l| {
                l * right
                    .iter()
                    .map(|r| if r == l { 1 } else { 0 })
                    .sum::<i32>()
            })
            .sum();
        Ok(result as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
