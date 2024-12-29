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
        let puzzle = Puzzle::new(&mut lines);
        // TODO: Solve Part 1 of the puzzle
        puzzle.dump_state();
        Ok(puzzle.solve())
    }

    assert_eq!(7036, part1(BufReader::new(TEST_1.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST_2.as_bytes()))?);

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
        }
    }

    fn dump_state(&self) {
        self.map.iter().enumerate().for_each(|(r_index, r)| {
            println!(
                "{}",
                r.iter()
                    .enumerate()
                    .map(|(t_index, t)| {
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

    fn solve(&self) -> usize {
        todo!()
    }
}
