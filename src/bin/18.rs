use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

#[derive(Debug)]
struct Puzzle {
    size: usize,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, PartialEq, Debug)]
enum Tile {
    Free,
    Ocuppied,
    Step(usize),
}

impl Puzzle {
    fn new(size: usize) -> Self {
        Self {
            size,
            tiles: vec![vec![Tile::Free; size]; size],
        }
    }

    fn push_byte(&mut self, row: usize, col: usize) {
        self.tiles[row][col] = Tile::Ocuppied;
    }

    fn solve(&mut self) -> bool {
        let mut tips: Vec<(usize, usize)> = vec![(0, 0)];
        let mut path_length = 0;
        loop {
            if tips.is_empty() {
                return false;
            }

            let mut new_tips = vec![];
            for (row, col) in tips {
                if self.tiles[row][col] == Tile::Free {
                    self.tiles[row][col] = Tile::Step(path_length);
                    if row + 1 < self.size {
                        new_tips.push((row + 1, col));
                    }
                    if col + 1 < self.size {
                        new_tips.push((row, col + 1));
                    }
                    if row > 0 {
                        new_tips.push((row - 1, col));
                    }
                    if col > 0 {
                        new_tips.push((row, col - 1));
                    }
                }
                if row == self.size - 1 && col == self.size - 1 {
                    return true;
                }
            }
            path_length += 1;
            tips = new_tips;
        }
    }

    fn shortest_path(&self) -> usize {
        match self.tiles[self.size - 1][self.size - 1] {
            Tile::Step(x) => x,
            _ => panic!("Couldn't find optimal path."),
        }
    }

    fn _dump_state(&self) {
        self.tiles.iter().for_each(|row| {
            row.iter().for_each(|t| match t {
                Tile::Free => print!("."),
                Tile::Ocuppied => print!("#"),
                Tile::Step(_) => print!("O"),
            });
            println!();
        });
    }

    fn clear(&mut self) {
        for row in 0..self.size {
            for col in 0..self.size {
                if matches!(self.tiles[row][col], Tile::Step(_)) {
                    self.tiles[row][col] = Tile::Free;
                }
            }
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(ram_size: usize, byte_count: usize, reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);

        let mut puzzle = Puzzle::new(ram_size);
        for _ in 0..byte_count {
            let coords: Vec<usize> = lines
                .next()
                .expect("Line was found")
                .split(",")
                .map(|v| v.parse::<usize>().expect("Parseable numer."))
                .collect();
            puzzle.push_byte(coords[1], coords[0]);
        }

        puzzle.solve();

        Ok(puzzle.shortest_path())
    }

    assert_eq!(22, part1(7, 12, BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(71, 1024, input_file)?);
    println!("Result = {}", result);
    assert_eq!(324, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(ram_size: usize, reader: R) -> Result<String> {
        let mut puzzle = Puzzle::new(ram_size);

        let mut lines = reader.lines().map_while(Result::ok);
        loop {
            let coords: Vec<usize> = lines
                .next()
                .expect("Line was found")
                .split(",")
                .map(|v| v.parse::<usize>().expect("Parseable numer."))
                .collect();

            puzzle.push_byte(coords[1], coords[0]);

            if !puzzle.solve() {
                return Ok(format!("{},{}", coords[0], coords[1]));
            }
            puzzle.clear();
        }
    }

    assert_eq!("6,1", part2(7, BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(71, input_file)?);
    println!("Result = {}", result);
    assert_eq!("46,23", result);
    Ok(())
}
