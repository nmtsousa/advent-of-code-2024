use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

struct Map {
    map: Vec<Vec<u8>>,
    row_count: usize,
    col_count: usize,
}

impl Map {
    fn new() -> Map {
        Map {
            map: vec![],
            row_count: 0,
            col_count: 0,
        }
    }

    fn add_row(&mut self, row: Vec<u8>) {
        if self.col_count == 0 {
            self.col_count = row.len();
        };

        self.row_count += 1;
        self.map.push(row);
    }

    fn sum_trailhead_scores(&self) -> usize {
        let mut score = 0;
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                if self.map[row][col] == 0 {
                    let mut found_endings: Vec<Vec<bool>> =
                        vec![vec![false; self.col_count]; self.row_count];
                    let trail_score = self.trailhead_score_from(row, col, &mut found_endings);
                    score += trail_score;
                }
            }
        }
        score
    }

    fn trailhead_score_from(
        &self,
        row: usize,
        col: usize,
        found_endings: &mut Vec<Vec<bool>>,
    ) -> usize {
        let current_height = self.map[row][col];
        if current_height == 9 {
            if found_endings[row][col] {
                return 0;
            }

            found_endings[row][col] = true;
            return 1;
        }

        let next_height = current_height + 1;

        let mut result = 0;

        if row > 0 && self.map[row - 1][col] == next_height {
            result += self.trailhead_score_from(row - 1, col, found_endings);
        }

        if row + 1 < self.row_count && self.map[row + 1][col] == next_height {
            result += self.trailhead_score_from(row + 1, col, found_endings);
        }

        if col > 0 && self.map[row][col - 1] == next_height {
            result += self.trailhead_score_from(row, col - 1, found_endings);
        }
        if col + 1 < self.col_count && self.map[row][col + 1] == next_height {
            result += self.trailhead_score_from(row, col + 1, found_endings);
        }

        result
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map = Map::new();
        for line in reader.lines().map_while(Result::ok) {
            map.add_row(
                line.chars()
                    .map(|c| c.to_digit(10).expect("Digit from 0 to 9") as u8)
                    .collect(),
            );
        }

        let result = map.sum_trailhead_scores();
        Ok(result)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

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
