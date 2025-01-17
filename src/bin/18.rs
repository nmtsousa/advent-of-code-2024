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
    fn new(size: usize, lines: &mut impl Iterator<Item = String>, byte_count: usize) -> Self {
        let mut tiles = vec![vec![Tile::Free; size]; size];
        for _ in 0..byte_count {
            let coords: Vec<usize> = lines
                .next()
                .expect("Line was found")
                .split(",")
                .into_iter()
                .map(|v| v.parse::<usize>().expect("Parseable numer."))
                .collect();
            tiles[coords[1]][coords[0]] = Tile::Ocuppied;
        }
        Self { size, tiles }
    }

    fn solve(&mut self) {
        if !self._try_solve(0, 0, 0) {
            panic!("Failed to solve puzzle.");
        }
    }

    fn _try_solve(&mut self, row: usize, col: usize, length: usize) -> bool {
        if row >= self.size || col >= self.size {
            return false;
        }
        match self.tiles[row][col] {
            Tile::Ocuppied => false,
            Tile::Step(x) if x < length => false,
            _ => {
                self.tiles[row][col] = Tile::Step(length);

                if row == self.size - 1 && col == self.size - 1 {
                    return true;
                }

                self._try_moving(row, col, length + 1);
                true
            }
        }
    }

    fn _try_moving(&mut self, row: usize, col: usize, length: usize) {
        self._try_solve(row, col + 1, length);
        self._try_solve(row + 1, col, length);
        if row > 0 {
            self._try_solve(row - 1, col, length);
        }
        if col > 0 {
            self._try_solve(row, col - 1, length);
        }
    }

    fn count_steps(&self) -> usize {
        match self.tiles[self.size - 1][self.size - 1] {
            Tile::Step(x) => x,
            _ => panic!("Couldn't find optimal path."),
        }
    }

    fn _dump_state(&self) {
        self.tiles.iter().for_each(|row| {
            let row: String = row
                .iter()
                .map(|t| match t {
                    Tile::Free => '.',
                    Tile::Ocuppied => '#',
                    Tile::Step(_) => 'O',
                })
                .collect();
            println!("{}", row);
        });
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(ram_size: usize, byte_count: usize, reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);
        println!("Reading puzzle...");
        let mut puzzle = Puzzle::new(ram_size, &mut lines, byte_count);
        println!("Solving puzzle...");
        puzzle.solve();

        Ok(puzzle.count_steps())
    }

    assert_eq!(22, part1(7, 12, BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(71, 1024, input_file)?);
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
