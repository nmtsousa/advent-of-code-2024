use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

#[derive(Debug)]
struct RaceTrack {
    row_count: usize,
    col_count: usize,
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<Tile>>,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Track,
    Path(usize),
}

impl RaceTrack {
    fn new(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut start = Option::None;
        let mut end = Option::None;
        let mut map: Vec<Vec<Tile>> = vec![];

        for (row_index, rowstr) in lines.enumerate() {
            map.push(
                rowstr
                    .chars()
                    .enumerate()
                    .map(|(col_index, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Track,
                        'S' => {
                            start = Some((row_index, col_index));
                            Tile::Track
                        }
                        'E' => {
                            end = Some((row_index, col_index));
                            Tile::Track
                        }
                        x => panic!("Unknown tile type {x}"),
                    })
                    .collect(),
            );
        }
        Self {
            row_count: map.len(),
            col_count: map[0].len(),
            start: start.expect("Start position expected in map"),
            end: end.expect("End position expected in map"),
            map,
        }
    }

    fn solve(&mut self) {
        let mut visited = vec![vec![false; self.col_count]; self.row_count];
        let mut tips = vec![Rc::new(PathStep::new(self.start))];
        let mut path_length = 0;

        loop {
            if tips.is_empty() {
                panic!("Can't find solution for race tracke.");
            }

            let mut new_tips = vec![];
            for step in tips {
                let (row, col) = step.pos;
                if !visited[row][col] && self.map[row][col] == Tile::Track {
                    visited[row][col] = true;
                    if row + 1 < self.row_count {
                        new_tips.push(Rc::new(PathStep::new_child((row + 1, col), &step)));
                    }
                    if col + 1 < self.col_count {
                        new_tips.push(Rc::new(PathStep::new_child((row, col + 1), &step)));
                    }
                    if row > 0 {
                        new_tips.push(Rc::new(PathStep::new_child((row - 1, col), &step)));
                    }
                    if col > 0 {
                        new_tips.push(Rc::new(PathStep::new_child((row, col - 1), &step)));
                    }
                }
                if (row, col) == self.end {
                    let mut current = Option::Some(step);
                    while let Some(s) = current {
                        self.map[s.pos.0][s.pos.1] = Tile::Path(path_length);
                        path_length -= 1;
                        current = s.parent.clone();
                    }

                    self._dump_track();
                    return;
                }
            }
            path_length += 1;
            tips = new_tips;
        }
    }

    fn _dump_track(&self) {
        self.map.iter().for_each(|row| {
            row.iter().for_each(|t| match t {
                Tile::Track => print!("."),
                Tile::Wall => print!("#"),
                Tile::Path(_) => print!("O"),
            });
            println!();
        });
    }

    fn count_cheats(&self, pico_seconds: usize) -> usize {
        let mut count = 0;
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                if let Tile::Path(this) = self.map[row][col] {
                    if row > 1 && self.map[row - 1][col] == Tile::Wall {
                        if let Tile::Path(other) = self.map[row - 2][col] {
                            if other > this && other - this - 2 >= pico_seconds {
                                count += 1;
                            }
                        }
                    }
                    if row + 2 < self.row_count && self.map[row + 1][col] == Tile::Wall {
                        if let Tile::Path(other) = self.map[row + 2][col] {
                            if other > this && other - this - 2 >= pico_seconds {
                                count += 1;
                            }
                        }
                    }
                    if col > 1 && self.map[row][col - 1] == Tile::Wall {
                        if let Tile::Path(other) = self.map[row][col - 2] {
                            if other > this && other - this - 2 >= pico_seconds {
                                count += 1;
                            }
                        }
                    }
                    if col + 2 < self.col_count && self.map[row][col + 1] == Tile::Wall {
                        if let Tile::Path(other) = self.map[row][col + 2] {
                            if other > this && other - this - 2 >= pico_seconds {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

#[derive(Debug)]
struct PathStep {
    pos: (usize, usize),
    parent: Option<Rc<PathStep>>,
}

impl PathStep {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            parent: Option::None,
        }
    }

    fn new_child<'a>(pos: (usize, usize), parent: &Rc<PathStep>) -> PathStep {
        PathStep {
            pos,
            parent: Option::Some(parent.clone()),
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(pico_seconds: usize, reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);

        let mut racetrack = RaceTrack::new(&mut lines);

        racetrack.solve();

        Ok(racetrack.count_cheats(pico_seconds))
    }

    // Set the expected answer for the test input
    assert_eq!(1, part1(64, BufReader::new(TEST.as_bytes()))?);
    assert_eq!(2, part1(40, BufReader::new(TEST.as_bytes()))?);
    assert_eq!(3, part1(38, BufReader::new(TEST.as_bytes()))?);
    assert_eq!(4, part1(36, BufReader::new(TEST.as_bytes()))?);
    assert_eq!(5, part1(20, BufReader::new(TEST.as_bytes()))?);
    assert_eq!(8, part1(12, BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(100, input_file)?);
    println!("Result = {}", result);
    assert_eq!(1323, result);
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
