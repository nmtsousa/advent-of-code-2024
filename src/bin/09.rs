use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut disk: Vec<i16> = vec![];

        let mut length_or_freespace = true;
        let mut field_id: i16 = 0;

        for line in reader.lines().map_while(Result::ok) {
            for c in line.chars() {
                let size = c
                    .to_string()
                    .parse::<usize>()
                    .expect("Digit is expected in input");

                if length_or_freespace {
                    for _ in 0..size {
                        disk.push(field_id);
                    }

                    field_id += 1;
                } else {
                    for _ in 0..size {
                        disk.push(-1);
                    }
                }
                length_or_freespace = !length_or_freespace;
            }
        }

        let mut last_free = disk.len() - 1;
        let mut next_free = 0;
        while disk[next_free] != -1 {
            next_free += 1;
        }

        while disk[last_free] == -1 {
            last_free -= 1;
        }

        while next_free < last_free {
            disk[next_free] = disk[last_free];
            disk[last_free] = -1;
            next_free += 1;
            last_free -= 1;

            while disk[next_free] != -1 {
                next_free += 1;
            }
            while disk[last_free] == -1 {
                last_free -= 1;
            }
        }

        let mut total = 0;

        for (i, v) in disk.iter().enumerate() {
            if *v != -1 {
                total += i * *v as usize;
            }
        }

        Ok(total)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

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
