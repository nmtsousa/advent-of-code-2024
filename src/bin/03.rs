use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use regex::Regex;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
const TEST_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result : usize = 0;
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
        for line in reader.lines() {
            result += re.captures_iter(&line.unwrap())
                .map(|cap| {
                    let a = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let b = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();

                    a * b
                }).sum::<usize>();
        }
        Ok(result)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result : usize = 0;
        let mut calc = true;
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)")?;
        for line in reader.lines() {
            result += re.captures_iter(&line.unwrap())
                .map(|cap| {
                    let mut a  = 0;
                    let mut b = 0;
                    match cap.get(0).unwrap().as_str() {
                        "do()" => calc = true,
                        "don't()" => calc = false,
                        _ => {
                            a = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                            b = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
                        }
                    };
                    if calc {
                        a * b
                    } else {
                        0
                    }
                }).sum::<usize>();
        }
        Ok(result)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST_2.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
