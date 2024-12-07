use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

struct CharMatrix {
    matrix: Vec<String>,
    row_count: usize,
    col_count: usize,
}

impl CharMatrix {
    fn new(matrix: Vec<String>) -> CharMatrix {
        let row_count = matrix.len();
        let col_count = matrix
            .first()
            .expect("There is a firt line in the problem")
            .len();
        CharMatrix {
            matrix,
            row_count,
            col_count,
        }
    }

    fn row_count(&self) -> usize {
        self.row_count
    }

    fn col_count(&self) -> usize {
        self.col_count
    }

    fn char_at(&self, row: usize, col: usize) -> &str {
        self.matrix
            .get(row)
            .expect("row exists")
            .get(col..col + 1)
            .unwrap()
    }

    fn find_string(
        &self,
        start_row: usize,
        start_col: usize,
        row_step: i32,
        col_step: i32,
        needle: &str,
    ) -> bool {
        let mut row: i32 = start_row as i32;
        let mut col: i32 = start_col as i32;
        let mut needle_part = needle.chars();

        while let Some(char) = needle_part.next() {
            if row < 0 || row >= self.row_count() as i32 {
                return false;
            }
            if col < 0 || col >= self.col_count() as i32 {
                return false;
            }

            if Some(char.to_string().as_str())
                != self
                    .matrix
                    .get(row as usize)
                    .and_then(|row| row.get(col as usize..col as usize + 1))
            {
                return false;
            }

            row += row_step;
            col += col_step;
        }

        true
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Vec<String> = reader.lines().map(|r| r.expect("Line was read.")).collect();

        let matrix = CharMatrix::new(input);
        let row_count = matrix.row_count();
        let col_count = matrix.col_count();
        let mut result = 0;

        for row in 0..row_count {
            for col in 0..col_count {
                if matrix.char_at(row, col) != "X" {
                    continue;
                }

                // horizontals
                if matrix.find_string(row, col, 0, 1, "XMAS") {
                    result += 1;
                }
                if matrix.find_string(row, col, 0, -1, "XMAS") {
                    result += 1;
                }

                // verticals
                if matrix.find_string(row, col, 1, 0, "XMAS") {
                    result += 1;
                }
                if matrix.find_string(row, col, -1, 0, "XMAS") {
                    result += 1;
                }

                // diagonals
                if matrix.find_string(row, col, 1, 1, "XMAS") {
                    result += 1;
                }
                if matrix.find_string(row, col, 1, -1, "XMAS") {
                    result += 1;
                }
                if matrix.find_string(row, col, -1, 1, "XMAS") {
                    result += 1;
                }
                if matrix.find_string(row, col, -1, -1, "XMAS") {
                    result += 1;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

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
