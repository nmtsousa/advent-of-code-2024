use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    col_count: usize,
    row_count: usize,
    antinode: Vec<Vec<char>>,
}

impl Map {
    fn new(first_row: String) -> Map {
        Map {
            map: vec![first_row.chars().collect()],
            col_count: first_row.len(),
            row_count: 1,
            antinode: vec![vec!['.'; first_row.len()]],
        }
    }

    fn push_row(&mut self, row: String) {
        self.map.push(row.chars().collect());
        self.antinode.push(vec!['.'; self.col_count]);
        self.row_count += 1;
    }

    fn count_antinodes(&self) -> usize {
        let mut count = 0;
        for row in &self.antinode {
            for char in row {
                if *char == '#' {
                    count += 1;
                }
            }
        }

        count
    }

    fn value_at(&self, row: usize, col: usize) -> char {
        self.map[row][col]
    }

    fn add_antinodes(&mut self, row_1: usize, col_1: usize, row_2: usize, col_2: usize) {
        let row_diff: i32 = row_2 as i32 - row_1 as i32;
        let col_diff: i32 = col_2 as i32 - col_1 as i32;

        self.add_antinode_at(row_1 as i32 - row_diff, col_1 as i32 - col_diff);
        self.add_antinode_at(row_2 as i32 + row_diff, col_2 as i32 + col_diff);
    }

    fn add_antinode_at(&mut self, row: i32, col: i32) -> bool {
        if row < 0 || row as usize >= self.row_count {
            return false;
        }

        if col < 0 || col as usize >= self.col_count {
            return false;
        }

        self.antinode[row as usize][col as usize] = '#';

        true
    }

    fn add_antinodes_2(&mut self, row_1: usize, col_1: usize, row_2: usize, col_2: usize) {
        let row_diff: i32 = row_2 as i32 - row_1 as i32;
        let col_diff: i32 = col_2 as i32 - col_1 as i32;

        self.add_antinodes_2_steps(row_1 as i32, col_1 as i32, -row_diff, -col_diff);
        self.add_antinodes_2_steps(row_2 as i32, col_2 as i32, row_diff, col_diff);
    }

    fn dump_state(&self) {
        self.map.iter().for_each(|row| {
            println!("{}", row.iter().collect::<String>());
        });
        println!()
    }

    fn add_antinodes_2_steps(&mut self, row: i32, col: i32, row_diff: i32, col_diff: i32) {
        let mut r = row - row_diff;
        let mut c = col - col_diff;
        while self.add_antinode_at(r, c) {
            r -= row_diff;
            c -= col_diff;
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Read map
        let mut lines = reader.lines().map_while(Result::ok);
        let first_row = lines.next().unwrap();
        let mut map = Map::new(first_row);

        for line in lines {
            map.push_row(line);
        }

        // for the whole map
        for row in 0..map.row_count {
            for col in 0..map.col_count {
                // find antenna symbol (ignore "." and "#")
                match map.value_at(row, col) {
                    '.' => continue,
                    '#' => continue,
                    antenna => {
                        // For reminder of map
                        //  Find same antenna symbol
                        for c in col + 1..map.col_count {
                            if antenna == map.value_at(row, c) {
                                map.add_antinodes(row, col, row, c);
                            }
                        }
                        for r in row + 1..map.row_count {
                            for c in 0..map.col_count {
                                if antenna == map.value_at(r, c) {
                                    map.add_antinodes(row, col, r, c);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Count antinodes
        let count = map.count_antinodes();
        Ok(count)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Read map
        let mut lines = reader.lines().map_while(Result::ok);
        let first_row = lines.next().unwrap();
        let mut map = Map::new(first_row);

        for line in lines {
            map.push_row(line);
        }

        // for the whole map
        for row in 0..map.row_count {
            for col in 0..map.col_count {
                // find antenna symbol (ignore "." and "#")
                match map.value_at(row, col) {
                    '.' => continue,
                    '#' => continue,
                    antenna => {
                        // For reminder of map
                        //  Find same antenna symbol
                        for c in col + 1..map.col_count {
                            if antenna == map.value_at(row, c) {
                                map.add_antinodes_2(row, col, row, c);
                            }
                        }
                        for r in row + 1..map.row_count {
                            for c in 0..map.col_count {
                                if antenna == map.value_at(r, c) {
                                    map.add_antinodes_2(row, col, r, c);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Count antinodes
        let count = map.count_antinodes();
        Ok(count)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
