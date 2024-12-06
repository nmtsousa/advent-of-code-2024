use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        for line in reader.lines() {
            let numbers: Vec<i32> = line?.split_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .collect();

            let mut num_iter = numbers.iter();
            let mut last = num_iter.next().unwrap();

            let mut valid = true;
            let mut direction = 0;
            for next in num_iter {
                let diff = next - last;
                if direction == 0 {
                    if diff > 0 {
                        direction = 1;
                    } else {
                        direction = -1;
                    }
                    if direction == 0 {
                        valid = false;
                        break;
                    }
                }

                let abs_diff = diff.abs();
                if abs_diff < 1 || abs_diff > 3 {
                    valid = false;
                    break;
                }

                if direction > 0 && diff < 0 {
                    valid = false;
                    break;
                }
                if direction < 0 && diff > 0 {
                    valid = false;
                    break;
                }
                last = next;
            }

            if valid {
                result += 1;
            }
        }
        
        Ok(result)
    }

    // Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

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
