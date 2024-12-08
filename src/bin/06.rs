use adv_code_2024::*;
use anyhow::*;
use bitflags::bitflags;
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

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Directions: u32 {
        const UP = 0b00000001;
        const RIGHT = 0b00000010;
        const DOWN = 0b00000100;
        const LEFT = 0b00001000;

        const UP_DOWN = Self::UP.bits() | Self::DOWN.bits();
        const LEFT_RIGTH = Self::LEFT.bits() | Self::RIGHT.bits();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Obstacle,
    Visited { dirs: Directions },
}

#[derive(Debug, PartialEq, Eq)]
enum SimulationResult {
    GuardExited,
    CycleDetected,
}

#[derive(Debug)]
struct Game {
    map: Vec<Vec<Tile>>,
    x: i32,
    y: i32,
    row_count: usize,
    col_count: usize,
    d: Directions,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self {
            map: self.map.iter().map(|row| row.to_vec()).collect(),
            x: self.x,
            y: self.y,
            row_count: self.row_count,
            col_count: self.col_count,
            d: self.d,
        }
    }
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
            row_count: 1,
            col_count: first_row.len(),
            d: Directions::UP,
        }
    }

    fn push_row(&mut self, row: String) {
        if let Some(pos) = row.find("^") {
            self.x = pos as i32;
            self.y = self.row_count as i32;
        }

        self.map.push(convert_row(&row));
        self.row_count += 1;
    }

    fn guard_in_map(&self) -> bool {
        self.in_map(self.x, self.y)
    }

    fn in_map(&self, test_x: i32, test_y: i32) -> bool {
        test_y >= 0
            && test_y < self.row_count as i32
            && test_x >= 0
            && test_x < self.col_count as i32
    }

    fn is_obstacle(&self, x: i32, y: i32) -> bool {
        self.in_map(x, y) && self.tile_is(x, y, Tile::Obstacle)
    }

    fn is_empty(&self, x: i32, y: i32) -> bool {
        self.in_map(x, y) && self.tile_is(x, y, Tile::Empty)
    }

    fn tile_is(&self, x: i32, y: i32, tile: Tile) -> bool {
        self.tile_at(x, y) == tile
    }

    fn tile_at(&self, x: i32, y: i32) -> Tile {
        self.map[y as usize][x as usize]
    }

    fn set_tile_at(&mut self, x: i32, y: i32, t: Tile) {
        self.map[y as usize][x as usize] = t;
    }

    fn tick(&mut self) -> Option<SimulationResult> {
        match self.tile_at(self.x, self.y) {
            Tile::Visited { dirs } => {
                if dirs.intersects(self.d) {
                    return Some(SimulationResult::CycleDetected);
                }
                self.set_tile_at(
                    self.x,
                    self.y,
                    Tile::Visited {
                        dirs: dirs | self.d,
                    },
                );
            }
            Tile::Obstacle => {
                self.dump_state();
                panic!("Can't tick on an obstacle.");
            }
            Tile::Empty => {
                self.set_tile_at(self.x, self.y, Tile::Visited { dirs: self.d });
            }
        }

        while self.next_is_obstacle() {
            match self.d {
                Directions::UP => self.d = Directions::RIGHT,
                Directions::RIGHT => self.d = Directions::DOWN,
                Directions::DOWN => self.d = Directions::LEFT,
                Directions::LEFT => self.d = Directions::UP,
                _ => panic!("Unexpected current direction."),
            }
        }

        match self.d {
            Directions::UP => self.y -= 1,
            Directions::RIGHT => self.x += 1,
            Directions::DOWN => self.y += 1,
            Directions::LEFT => self.x -= 1,
            _ => panic!("Unexpected current direction."),
        };

        None
    }

    fn next_is_obstacle(&self) -> bool {
        match self.d {
            Directions::UP => self.is_obstacle(self.x, self.y - 1),
            Directions::RIGHT => self.is_obstacle(self.x + 1, self.y),
            Directions::DOWN => self.is_obstacle(self.x, self.y + 1),
            Directions::LEFT => self.is_obstacle(self.x - 1, self.y),
            _ => panic!("Unexpected current direction."),
        }
    }

    fn count_visited(&self) -> usize {
        let mut result: usize = 0;
        for row in &self.map {
            for tile in row {
                if let Tile::Visited { dirs: _ } = *tile {
                    result += 1
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
                    Tile::Visited { dirs } => {
                        if dirs.intersects(Directions::UP_DOWN)
                            && dirs.intersects(Directions::LEFT_RIGTH)
                        {
                            "+"
                        } else if dirs.intersects(Directions::UP_DOWN) {
                            "|"
                        } else if dirs.intersects(Directions::LEFT_RIGTH) {
                            return "-";
                        } else {
                            "X"
                        }
                    }
                })
                .join("");
            println!("{}", row_str);
        }
        println!();
    }

    fn run_simulation(&mut self) -> SimulationResult {
        while self.guard_in_map() {
            if let Some(result) = self.tick() {
                return result;
            }
        }

        SimulationResult::GuardExited
    }

    fn count_obstacles_that_produce_cycles(&mut self) -> usize {
        let mut result: usize = 0;

        // Brute force version

        // Intelligent version
        while self.guard_in_map() {
            let mut copy = self.clone();
            if copy.insert_obstacle() && (copy.run_simulation() == SimulationResult::CycleDetected)
            {
                result += 1;
            }
            self.tick();
        }

        result
    }

    fn insert_obstacle(&mut self) -> bool {
        let mut obs_x = self.x;
        let mut obs_y = self.y;

        match self.d {
            Directions::UP => obs_y -= 1,
            Directions::RIGHT => obs_x += 1,
            Directions::DOWN => obs_y += 1,
            Directions::LEFT => obs_x -= 1,
            _ => panic!("Unexpected current direction."),
        };

        if !self.is_empty(obs_x, obs_y) {
            return false;
        }

        self.set_tile_at(obs_x, obs_y, Tile::Obstacle);

        true
    }
}

fn convert_row(row: &str) -> Vec<Tile> {
    let row: Vec<Tile> = row
        .chars()
        .map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            '^' => Tile::Empty,
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

        assert_eq!(SimulationResult::GuardExited, game.run_simulation());

        Ok(game.count_visited())
    }
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ==="); // 1866 <-> 194.

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);
        let first_row = lines.next().unwrap();
        let mut game = Game::new(first_row);

        for line in lines {
            game.push_row(line);
        }

        Ok(game.count_obstacles_that_produce_cycles())
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
