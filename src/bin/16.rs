use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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

    let test1_result = part1(BufReader::new(TEST_1.as_bytes()))?;
    assert_eq!(7036, test1_result.get_lower_cost());
    assert_eq!(45, test1_result.count_tiles_in_best_paths());

    let test2_result = part1(BufReader::new(TEST_2.as_bytes()))?;
    assert_eq!(11048, test2_result.get_lower_cost());
    assert_eq!(64, test2_result.count_tiles_in_best_paths());

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let puzzle_result = time_snippet!(part1(input_file)?);

    println!("=== Part 1 ===");
    println!("Result = {}", puzzle_result.get_lower_cost());
    assert_eq!(95476, puzzle_result.get_lower_cost());

    println!("\n=== Part 2 ===");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,        // #
    Free,        // .
    Start,       // S
    End,         // E
    Cost(usize), // Cost to reach tile
}

struct Puzzle {
    map: Vec<Vec<Tile>>,
    start_x: usize,
    start_y: usize,
    lowest_cost: Option<usize>,
    best_path_tile: Vec<Vec<bool>>,
}

#[derive(Debug)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
    cost: usize,
}

#[derive(Debug)]
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
        let new_cost = self.cost + TURN_COST + FORWARD_COST;
        match self.direction {
            Direction::East => State {
                x: self.x,
                y: self.y + 1,
                direction: Direction::South,
                cost: new_cost,
            },
            Direction::North => State {
                x: self.x - 1,
                y: self.y,
                direction: Direction::East,
                cost: new_cost,
            },
            Direction::West => State {
                x: self.x,
                y: self.y - 1,
                direction: Direction::North,
                cost: new_cost,
            },
            Direction::South => State {
                x: self.x + 1,
                y: self.y,
                direction: Direction::West,
                cost: new_cost,
            },
        }
    }

    fn turn_right(&self) -> State {
        let new_cost = self.cost + TURN_COST + FORWARD_COST;
        match self.direction {
            Direction::East => State {
                x: self.x,
                y: self.y - 1,
                direction: Direction::North,
                cost: new_cost,
            },
            Direction::North => State {
                x: self.x + 1,
                y: self.y,
                direction: Direction::West,
                cost: new_cost,
            },
            Direction::West => State {
                x: self.x,
                y: self.y + 1,
                direction: Direction::South,
                cost: new_cost,
            },
            Direction::South => State {
                x: self.x - 1,
                y: self.y,
                direction: Direction::East,
                cost: new_cost,
            },
        }
    }

    fn u_turn(&self) -> State {
        let new_cost = self.cost + TURN_COST + FORWARD_COST;
        match self.direction {
            Direction::East => State {
                x: self.x + 1,
                y: self.y,
                direction: Direction::West,
                cost: new_cost,
            },
            Direction::North => State {
                x: self.x,
                y: self.y + 1,
                direction: Direction::South,
                cost: new_cost,
            },
            Direction::West => State {
                x: self.x - 1,
                y: self.y,
                direction: Direction::East,
                cost: new_cost,
            },
            Direction::South => State {
                x: self.x,
                y: self.y - 1,
                direction: Direction::North,
                cost: new_cost,
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
                            Tile::Cost(_) => '+',
                        }
                    })
                    .collect::<String>()
            );
        });
    }

    fn solve(&mut self) {
        let start = State::new_start(self.start_x, self.start_y);
        let mut path = vec![];
        self.solve_for(start, &mut path);
    }

    fn get_lower_cost(&self) -> usize {
        self.lowest_cost.expect("Found lowest cost path.")
    }

    fn count_tiles_in_best_paths(&self) -> usize {
        self.dump_best_paths();

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

    fn solve_for(&mut self, state: State, path: &mut Vec<(usize, usize)>) {
        path.push((state.x, state.y));
        match self.map[state.y][state.x] {
            Tile::Free => self.map[state.y][state.x] = Tile::Cost(state.cost),
            Tile::Start => (),
            Tile::Cost(x) if state.cost < x => self.map[state.y][state.x] = Tile::Cost(state.cost),
            Tile::Cost(_) => {
                path.pop();
                return;
            }
            Tile::Wall => {
                path.pop();
                return;
            }
            Tile::End => {
                println!("End found cost {} , len {}", state.cost, path.len());
                match self.lowest_cost {
                    None => {
                        println!("First path found cost {}, len {}", state.cost, path.len());
                        self.lowest_cost = Some(state.cost);
                        self.mark_best_path(path);
                    }
                    Some(x) if x > state.cost => {
                        self.reset_best_path();
                        println!("Path found {}, len {}", state.cost, path.len());
                        self.mark_best_path(path);
                        self.lowest_cost = Some(state.cost);
                    }
                    Some(x) if x == state.cost => {
                        println!("Secondary path found {}, len {}", state.cost, path.len());
                        self.mark_best_path(path);
                    }
                    _ => (),
                };
                path.pop();
                return;
            }
        };

        self.solve_for(state.move_forward(), path);
        self.solve_for(state.turn_left(), path);
        self.solve_for(state.turn_right(), path);
        self.solve_for(state.u_turn(), path);
        path.pop();
    }

    fn reset_best_path(&mut self) {
        self.best_path_tile.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|v| {
                *v = false;
            });
        });
    }

    fn mark_best_path(&mut self, path: &[(usize, usize)]) {
        for (x, y) in path {
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
                                Tile::Cost(_) => '+',
                            }
                        }
                    })
                    .collect::<String>()
            );
        });
    }
}
