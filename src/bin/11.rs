use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut stone_map: HashMap<usize, usize> = HashMap::new();

        reader
            .lines()
            .map_while(Result::ok)
            .flat_map(|s| {
                s.split_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .map(|s| s.parse::<usize>())
            .map_while(Result::ok)
            .for_each(|s| {
                match stone_map.get(&s) {
                    None => stone_map.insert(s, 1),
                    Some(v) => stone_map.insert(s, v + 1),
                };
            });

        for _ in 0..25 {
            let new_stones: Vec<(usize, usize)> = stone_map
                .iter()
                .flat_map(|(k, v)| match k {
                    0 => vec![(1, *v)],
                    n if n.to_string().len() % 2 == 0 => {
                        let str = n.to_string();
                        let (left, right) = str.split_at(str.len() / 2);
                        vec![
                            (left.parse::<usize>().expect("Number"), *v),
                            (right.parse::<usize>().expect("Number"), *v),
                        ]
                    }
                    n => vec![(n * 2024, *v)],
                })
                .collect();

            stone_map = HashMap::new();

            new_stones.iter().for_each(|(k, v)| {
                match stone_map.get(k) {
                    None => stone_map.insert(*k, *v),
                    Some(e) => stone_map.insert(*k, *e + *v),
                };
            });
        }

        let mut result: usize = 0;
        stone_map.values().for_each(|c| {
            result += c;
        });

        Ok(result)
    }

    // Set the expected answer for the test input
    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(235850, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut stone_map: HashMap<usize, usize> = HashMap::new();

        reader
            .lines()
            .map_while(Result::ok)
            .flat_map(|s| {
                s.split_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .map(|s| s.parse::<usize>())
            .map_while(Result::ok)
            .for_each(|s| {
                match stone_map.get(&s) {
                    None => stone_map.insert(s, 1),
                    Some(v) => stone_map.insert(s, v + 1),
                };
            });

        for _ in 0..75 {
            let new_stones: Vec<(usize, usize)> = stone_map
                .iter()
                .flat_map(|(k, v)| match k {
                    0 => vec![(1, *v)],
                    n if n.to_string().len() % 2 == 0 => {
                        let str = n.to_string();
                        let (left, right) = str.split_at(str.len() / 2);
                        vec![
                            (left.parse::<usize>().expect("Number"), *v),
                            (right.parse::<usize>().expect("Number"), *v),
                        ]
                    }
                    n => vec![(n * 2024, *v)],
                })
                .collect();

            stone_map = HashMap::new();

            new_stones.iter().for_each(|(k, v)| {
                match stone_map.get(k) {
                    None => stone_map.insert(*k, *v),
                    Some(e) => stone_map.insert(*k, *e + *v),
                };
            });
        }

        let mut result: usize = 0;
        stone_map.values().for_each(|c| {
            result += c;
        });

        Ok(result)
    }

    assert_eq!(65601038650482, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
