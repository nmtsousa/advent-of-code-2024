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

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);
        let mut puzzle = Puzzle::new(&mut lines);
        let result = puzzle.solve();
        Ok(result)
    }

    assert_eq!(7036, part1(BufReader::new(TEST_1.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(95476, result);
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
        let mut map = vec![];
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

        Self {
            map,
            start_x: start_x.expect("Start X was found."),
            start_y: start_y.expect("Start Y was found."),
            lowest_cost: None,
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

    fn solve(&mut self) -> usize {
        // Start has a cost of 0

        let start = State::new_start(self.start_x, self.start_y);
        self.solve_for(start);

        self.lowest_cost.expect("Found lowest cost path.")
    }

    fn solve_for(&mut self, state: State) {
        match self.map[state.y][state.x] {
            Tile::Wall => return,
            Tile::Free => self.map[state.y][state.x] = Tile::Cost(state.cost),
            Tile::Start => (),
            Tile::Cost(x) if state.cost < x => self.map[state.y][state.x] = Tile::Cost(state.cost),
            Tile::Cost(_) => return,
            Tile::End => {
                match self.lowest_cost {
                    None => self.lowest_cost = Some(state.cost),
                    Some(x) if x > state.cost => self.lowest_cost = Some(state.cost),
                    _ => (),
                };
            }
        };

        self.solve_for(state.move_forward());
        self.solve_for(state.turn_left());
        self.solve_for(state.turn_right());
        self.solve_for(state.u_turn());
    }
}
