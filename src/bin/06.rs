use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Game {
    map: Vec<Vec<Tile>>,
    x: i32,
    y: i32,
    d: Direction,
}

impl Game {
    fn new(first_row: String) -> Game {
        let mut x: i32 = -1;
        let mut y: i32 = -1;

        if let Some(pos) = first_row.find("^") {
            x = pos as i32;
            y = 0;
        }

        Game {
            map: vec![convert_row(&first_row)],
            x,
            y,
            d: Direction::Up,
        }
    }

    fn push_row(&mut self, row: String) {
        if let Some(pos) = row.find("^") {
            self.x = pos as i32;
            self.y = self.map.len() as i32;
        }

        self.map.push(convert_row(&row));
    }

    fn guard_in_map(&self) -> bool {
        self.in_map(self.x, self.y)
    }

    fn in_map(&self, test_x: i32, test_y: i32) -> bool {
        test_y >= 0
            && test_y < self.map[0].len() as i32
            && test_x >= 0
            && test_x < self.map.len() as i32
    }

    fn is_obstacle(&self, x: i32, y: i32) -> bool {
        self.in_map(x, y) && self.map[y as usize][x as usize] == Tile::Obstacle
    }

    fn tick(&mut self) {
        self.map[self.y as usize][self.x as usize] = Tile::Visited;

        match self.d {
            Direction::Up => {
                if self.is_obstacle(self.x, self.y - 1) {
                    self.d = Direction::Right;
                    self.x += 1;
                } else {
                    self.y -= 1;
                }
            }
            Direction::Right => {
                if self.is_obstacle(self.x + 1, self.y) {
                    self.d = Direction::Down;
                    self.y += 1;
                } else {
                    self.x += 1;
                }
            }
            Direction::Down => {
                if self.is_obstacle(self.x, self.y + 1) {
                    self.d = Direction::Left;
                    self.x -= 1;
                } else {
                    self.y += 1;
                }
            }
            Direction::Left => {
                if (self.is_obstacle(self.x - 1, self.y)) {
                    self.d = Direction::Up;
                    self.y -= 1;
                } else {
                    self.x -= 1;
                }
            }
        }
    }

    fn count_visited(&self) -> usize {
        let mut result: usize = 0;
        for row in &self.map {
            for tile in row {
                if *tile == Tile::Visited {
                    result += 1;
                }
            }
        }

        result
    }

    fn dump_state(&self) {
        for row in &self.map {
            let row_str = row
                .iter()
                .map(|tile| match tile {
                    Tile::Empty => ".",
                    Tile::Obstacle => "#",
                    Tile::Visited => "X",
                })
                .join("");
            println!("{}", row_str);
        }
        println!();
    }
}

fn convert_row(row: &str) -> Vec<Tile> {
    let row: Vec<Tile> = row
        .chars()
        .map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            '^' => Tile::Obstacle,
            _ => panic!("Unexpected input!"),
        })
        .collect();
    row
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);
        let first_row = lines.next().unwrap();
        let mut game = Game::new(first_row);

        for line in lines {
            game.push_row(line);
        }

        while game.guard_in_map() {
            game.tick();
        }

        Ok(game.count_visited())
    }
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

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
