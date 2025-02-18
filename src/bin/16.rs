use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

const TEST_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

fn main() -> Result<()> {
    start_day(DAY);

    fn part1<R: BufRead>(reader: R) -> Result<Puzzle> {
        let mut lines = reader.lines().map_while(Result::ok);
        let mut puzzle = Puzzle::new(&mut lines);
        puzzle.solve();

        Ok(puzzle)
    }

    println!("=== Test 1 ===");
    let test1_result = time_snippet!(part1(BufReader::new(TEST_1.as_bytes()))?);
    assert_eq!(7036, test1_result.get_lower_cost());
    assert_eq!(45, test1_result.count_tiles_in_best_paths());

    println!("=== Test 2 ===");
    let test2_result = time_snippet!(part1(BufReader::new(TEST_2.as_bytes()))?);
    assert_eq!(11048, test2_result.get_lower_cost());
    assert_eq!(64, test2_result.count_tiles_in_best_paths());

    println!("=== Puzzle ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let puzzle_result = time_snippet!(part1(input_file)?);

    println!("=== Part 1 ===");
    println!("Result = {}", puzzle_result.get_lower_cost());
    assert_eq!(95476, puzzle_result.get_lower_cost());

    println!("\n=== Part 2 ===");

    println!("Result = {}", puzzle_result.count_tiles_in_best_paths());
    assert_eq!(511, puzzle_result.count_tiles_in_best_paths());

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,  // #
    Free,  // .
    Start, // S
    End,   // E
}

struct Puzzle {
    map: Vec<Vec<Tile>>,
    best_path_tile: Vec<Vec<bool>>,
    start_x: usize,
    start_y: usize,
    lowest_cost: Option<usize>,
}

#[derive(Debug)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
    cost: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    East,
    North,
    West,
    South,
}

const FORWARD_COST: usize = 1;
const TURN_COST: usize = 1000;

impl State {
    fn new_start(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::East,
            cost: 0,
        }
    }

    fn move_forward(&self) -> State {
        match self.direction {
            Direction::East => State {
                x: self.x - 1,
                y: self.y,
                direction: Direction::East,
                cost: self.cost + FORWARD_COST,
            },
            Direction::North => State {
                x: self.x,
                y: self.y - 1,
                direction: Direction::North,
                cost: self.cost + FORWARD_COST,
            },
            Direction::West => State {
                x: self.x + 1,
                y: self.y,
                direction: Direction::West,
                cost: self.cost + FORWARD_COST,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
                direction: Direction::South,
                cost: self.cost + FORWARD_COST,
            },
        }
    }

    fn turn_left(&self) -> State {
        State {
            x: self.x,
            y: self.y,
            cost: self.cost + TURN_COST,
            direction: match self.direction {
                Direction::East => Direction::South,
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
            },
        }
    }

    fn turn_right(&self) -> State {
        State {
            x: self.x,
            y: self.y,
            cost: self.cost + TURN_COST,
            direction: match self.direction {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
            },
        }
    }

    fn u_turn(&self) -> State {
        State {
            x: self.x,
            y: self.y,
            cost: self.cost + 2 * TURN_COST,
            direction: match self.direction {
                Direction::East => Direction::West,
                Direction::North => Direction::South,
                Direction::West => Direction::East,
                Direction::South => Direction::North,
            },
        }
    }
}

impl Puzzle {
    fn new(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut map: Vec<Vec<Tile>> = vec![];
        let mut start_x = None;
        let mut start_y = None;

        for (y, line) in lines.by_ref().enumerate() {
            if line.is_empty() {
                break;
            }
            map.push(
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Free,
                        'S' => {
                            start_x = Some(x);
                            start_y = Some(y);
                            Tile::Start
                        }
                        'E' => Tile::End,
                        _ => panic!("Unexpected tile"),
                    })
                    .collect(),
            );
        }

        let row_count = map.len();
        let col_count = map[0].len();

        Self {
            map,
            start_x: start_x.expect("Start X was found."),
            start_y: start_y.expect("Start Y was found."),
            lowest_cost: None,
            best_path_tile: vec![vec![false; col_count]; row_count],
        }
    }

    fn dump_state(&self) {
        self.map.iter().for_each(|r| {
            println!(
                "{}",
                r.iter()
                    .map(|t| {
                        match t {
                            Tile::Wall => '#',
                            Tile::Free => '.',
                            Tile::Start => 'S',
                            Tile::End => 'E',
                        }
                    })
                    .collect::<String>()
            );
        });
    }

    fn solve(&mut self) {
        let start = State::new_start(self.start_x, self.start_y);
        let mut tracker = PathTracker::new();
        self.solve_for(start, &mut tracker);
    }

    fn get_lower_cost(&self) -> usize {
        self.lowest_cost.expect("Found lowest cost path.")
    }

    fn count_tiles_in_best_paths(&self) -> usize {
        let mut count = 0;
        self.best_path_tile.iter().for_each(|row| {
            row.iter().for_each(|v| {
                if *v {
                    count += 1;
                }
            });
        });
        count
    }

    fn solve_for(&mut self, state: State, tracker: &mut PathTracker) {
        if let Some(cost) = self.lowest_cost {
            if cost < state.cost {
                return;
            }
        }

        if !tracker.is_worh_continuing(&state) {
            return;
        }

        tracker.push(&state);
        match self.map[state.y][state.x] {
            Tile::Wall => {
                tracker.pop();
                return;
            }
            Tile::End => {
                match self.lowest_cost {
                    None => {
                        self.lowest_cost = Some(state.cost);
                        self.mark_best_path(tracker);
                    }
                    Some(x) if x > state.cost => {
                        self.reset_best_path();
                        self.mark_best_path(tracker);
                        self.lowest_cost = Some(state.cost);
                    }
                    Some(x) if x == state.cost => {
                        self.mark_best_path(tracker);
                    }
                    _ => (),
                };
                tracker.pop();
                return;
            }
            _ => {}
        };

        self.solve_for(state.move_forward(), tracker);
        self.solve_for(state.turn_left(), tracker);
        self.solve_for(state.turn_right(), tracker);
        self.solve_for(state.u_turn(), tracker);
        tracker.pop();
    }

    fn reset_best_path(&mut self) {
        self.best_path_tile.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|v| {
                *v = false;
            });
        });
    }

    fn mark_best_path(&mut self, tracker: &mut PathTracker) {
        for (x, y) in tracker.path.iter() {
            self.best_path_tile[*y][*x] = true;
        }
    }

    fn dump_best_paths(&self) {
        self.map.iter().enumerate().for_each(|(row, r)| {
            println!(
                "{}",
                r.iter()
                    .enumerate()
                    .map(|(col, t)| {
                        if self.best_path_tile[row][col] {
                            'O'
                        } else {
                            match t {
                                Tile::Wall => '#',
                                Tile::Free => '.',
                                Tile::Start => 'S',
                                Tile::End => 'E',
                            }
                        }
                    })
                    .collect::<String>()
            );
        });
    }
}

struct PathTracker {
    path: Vec<(usize, usize)>,
    cost_per_direction: HashMap<(usize, usize), HashMap<Direction, usize>>,
}

impl PathTracker {
    fn new() -> Self {
        Self {
            path: vec![],
            cost_per_direction: HashMap::new(),
        }
    }

    fn push(&mut self, state: &State) {
        self.path.push((state.x, state.y));

        self.cost_per_direction
            .entry((state.x, state.y))
            .or_insert(HashMap::new())
            .entry(state.direction)
            .and_modify(|c| {
                if *c > state.cost {
                    *c = state.cost;
                }
            })
            .or_insert(state.cost);
    }

    fn pop(&mut self) {
        self.path.pop().expect("Point to pop is there.");
    }

    fn is_worh_continuing(&self, state: &State) -> bool {
        //!self.visited[state.y][state.x]
        match self.cost_per_direction.get(&(state.x, state.y)) {
            None => true,
            Some(map) => match map.get(&state.direction) {
                None => true,
                Some(c) => *c >= state.cost,
            },
        }
    }
}
