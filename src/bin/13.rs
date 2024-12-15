use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;

        let mut input = reader.lines().map_while(Result::ok);
        while let Some(line) = input.next() {
            if line.starts_with("Button A") {
                let mut re = Regex::new(r"^Button A\: X\+([0-9]+), Y\+([0-9]+)$")?;
                let mut captures = re.captures(&line).expect("Button A captured.");
                let a_x = captures[1].parse::<usize>().expect("Number for X");
                let a_y = captures[2].parse::<usize>().expect("Number for Y");

                let line = input.next().expect("Line for B");
                re = Regex::new(r"^Button B\: X\+([0-9]+), Y\+([0-9]+)$")?;
                captures = re.captures(&line).expect("Button B captured.");
                let b_x = captures[1].parse::<usize>().expect("Number for X");
                let b_y = captures[2].parse::<usize>().expect("Number for Y");

                let line = input.next().expect("Line for Prize");
                re = Regex::new(r"^Prize\: X=([0-9]+), Y=([0-9]+)$")?;
                captures = re.captures(&line).expect("Prize captured.");
                let target_x = captures[1].parse::<usize>().expect("Number for X");
                let target_y = captures[2].parse::<usize>().expect("Number for Y");

                result += compute_coins_part1(target_x, target_y, a_x, a_y, b_x, b_y);
            }
        }

        Ok(result)
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;

        let mut input = reader.lines().map_while(Result::ok);
        while let Some(line) = input.next() {
            if line.starts_with("Button A") {
                let mut re = Regex::new(r"^Button A\: X\+([0-9]+), Y\+([0-9]+)$")?;
                let mut captures = re.captures(&line).expect("Button A captured.");
                let a_x = captures[1].parse::<isize>().expect("Number for X");
                let a_y = captures[2].parse::<isize>().expect("Number for Y");

                let line = input.next().expect("Line for B");
                re = Regex::new(r"^Button B\: X\+([0-9]+), Y\+([0-9]+)$")?;
                captures = re.captures(&line).expect("Button B captured.");
                let b_x = captures[1].parse::<isize>().expect("Number for X");
                let b_y = captures[2].parse::<isize>().expect("Number for Y");

                let line = input.next().expect("Line for Prize");
                re = Regex::new(r"^Prize\: X=([0-9]+), Y=([0-9]+)$")?;
                captures = re.captures(&line).expect("Prize captured.");
                let target_x = captures[1].parse::<isize>().expect("Number for X") + 10000000000000;
                let target_y = captures[2].parse::<isize>().expect("Number for Y") + 10000000000000;

                result += compute_coins_part2(a_x, a_y, b_x, b_y, target_x, target_y);
            }
        }

        Ok(result)
    }

    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn compute_coins_part1(
    target_x: usize,
    target_y: usize,
    a_x: usize,
    a_y: usize,
    b_x: usize,
    b_y: usize,
) -> usize {
    // A -> 3 tokens
    // B -> 1 token

    // Max each button press 100

    for b in 0..100 {
        for a in 0..100 {
            if target_x == b * b_x + a * a_x && target_y == b * b_y + a * a_y {
                return a * 3 + b;
            }
        }
    }

    0
}

fn compute_coins_part2(
    a_x: isize,
    a_y: isize,
    b_x: isize,
    b_y: isize,
    prize_x: isize,
    prize_y: isize,
) -> usize {
    let det = a_x * b_y - a_y * b_x;
    let a = (prize_x * b_y - prize_y * b_x) / det;
    let b = (a_x * prize_y - a_y * prize_x) / det;
    if (a_x * a + b_x * b, a_y * a + b_y * b) == (prize_x, prize_y) {
        (a * 3 + b) as usize
    } else {
        0
    }
}
