use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

const TEST_2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

const TEST_3: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);
        let mut wharehouse = Wharehouse::new(&mut lines);

        for line in lines.by_ref() {
            line.chars().for_each(|c| {
                //println!("Move: {}", c);
                match c {
                    '<' => wharehouse.move_left(),
                    '>' => wharehouse.move_right(),
                    '^' => wharehouse.move_up(),
                    'v' => wharehouse.move_down(),
                    c => panic!("Unexpected motion [{}]!", c),
                };
                //wharehouse.dump_state();
            });
        }

        Ok(wharehouse.sum_gps())
    }

    assert_eq!(2028, part1(BufReader::new(TEST_1.as_bytes()))?);
    assert_eq!(10092, part1(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);
        let mut wharehouse = WharehouseWide::new(&mut lines);

        // wharehouse.dump_state();
        for line in lines.by_ref() {
            line.chars().for_each(|c| {
                // println!("Move: {}", c);
                match c {
                    '<' => wharehouse.move_left(),
                    '>' => wharehouse.move_right(),
                    '^' => wharehouse.move_up(),
                    'v' => wharehouse.move_down(),
                    c => panic!("Unexpected motion [{}]!", c),
                };
                // wharehouse.dump_state();
                wharehouse.validate_state();
            });
        }

        Ok(wharehouse.sum_gps())
    }

    assert_eq!(1751, part2(BufReader::new(TEST_1.as_bytes()))?);
    assert_eq!(9021, part2(BufReader::new(TEST_2.as_bytes()))?);
    assert_eq!(618, part2(BufReader::new(TEST_3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug)]
struct Wharehouse {
    map: Vec<Vec<Tile>>,
    robot_x: usize,
    robot_y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall, // #
    Free, // .
    Box,  // O
}

impl Wharehouse {
    fn new(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut map = vec![];
        let mut robot_x = None;
        let mut robot_y = None;

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
                        'O' => Tile::Box,
                        '@' => {
                            robot_x = Some(x);
                            robot_y = Some(y);
                            Tile::Free
                        }
                        _ => panic!("Unexpected tile"),
                    })
                    .collect(),
            );
        }

        Self {
            map,
            robot_x: robot_x.expect("Robot X"),
            robot_y: robot_y.expect("Rboot Y"),
        }
    }

    fn move_left(&mut self) {
        if self.attemp_left(self.robot_x - 1, self.robot_y) {
            self.robot_x -= 1;
        }
    }

    fn attemp_left(&mut self, target_x: usize, target_y: usize) -> bool {
        match self.get_at(target_x, target_y) {
            Tile::Free => true,
            Tile::Wall => false,
            Tile::Box if self.attemp_left(target_x - 1, target_y) => {
                self.map[target_y][target_x - 1] = Tile::Box;
                self.map[target_y][target_x] = Tile::Free;
                true
            }
            _ => false,
        }
    }

    fn get_at(&self, target_x: usize, target_y: usize) -> Tile {
        self.map[target_y][target_x]
    }

    fn move_right(&mut self) {
        if self.attemp_right(self.robot_x + 1, self.robot_y) {
            self.robot_x += 1;
        }
    }

    fn attemp_right(&mut self, target_x: usize, target_y: usize) -> bool {
        match self.get_at(target_x, target_y) {
            Tile::Free => true,
            Tile::Wall => false,
            Tile::Box if self.attemp_right(target_x + 1, target_y) => {
                self.map[target_y][target_x + 1] = Tile::Box;
                self.map[target_y][target_x] = Tile::Free;
                true
            }
            _ => false,
        }
    }

    fn move_up(&mut self) {
        if self.attemp_up(self.robot_x, self.robot_y - 1) {
            self.robot_y -= 1;
        }
    }

    fn attemp_up(&mut self, target_x: usize, target_y: usize) -> bool {
        match self.get_at(target_x, target_y) {
            Tile::Free => true,
            Tile::Wall => false,
            Tile::Box if self.attemp_up(target_x, target_y - 1) => {
                self.map[target_y - 1][target_x] = Tile::Box;
                self.map[target_y][target_x] = Tile::Free;
                true
            }
            _ => false,
        }
    }

    fn move_down(&mut self) {
        if self.attemp_down(self.robot_x, self.robot_y + 1) {
            self.robot_y += 1;
        }
    }

    fn attemp_down(&mut self, target_x: usize, target_y: usize) -> bool {
        match self.get_at(target_x, target_y) {
            Tile::Free => true,
            Tile::Wall => false,
            Tile::Box if self.attemp_down(target_x, target_y + 1) => {
                self.map[target_y + 1][target_x] = Tile::Box;
                self.map[target_y][target_x] = Tile::Free;
                true
            }
            _ => false,
        }
    }

    fn sum_gps(&self) -> usize {
        let mut result = 0;

        self.map.iter().enumerate().for_each(|(row_index, row)| {
            row.iter().enumerate().for_each(|(col_index, tile)| {
                if *tile == Tile::Box {
                    result += row_index * 100 + col_index;
                }
            });
        });

        result
    }

    fn dump_state(&self) {
        self.map.iter().enumerate().for_each(|(r_index, r)| {
            println!(
                "{}",
                r.iter()
                    .enumerate()
                    .map(|(t_index, t)| {
                        if r_index == self.robot_y && t_index == self.robot_x {
                            return '@';
                        }
                        match t {
                            Tile::Box => 'O',
                            Tile::Free => '.',
                            Tile::Wall => '#',
                        }
                    })
                    .collect::<String>()
            );
        });
    }
}

#[derive(Debug)]
struct WharehouseWide {
    map: Vec<Vec<TileWide>>,
    robot_x: usize,
    robot_y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileWide {
    Wall,     // #
    Free,     // .
    BoxLeft,  // [
    BoxRight, // ]
}

impl WharehouseWide {
    fn new(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut map = vec![];
        let mut robot_x = None;
        let mut robot_y = None;

        for (y, line) in lines.by_ref().enumerate() {
            if line.is_empty() {
                break;
            }

            map.push(
                line.chars()
                    .flat_map(|c| {
                        match c {
                            '#' => "##",
                            'O' => "[]",
                            '.' => "..",
                            '@' => "@.",
                            _ => panic!("Unexpected input to double."),
                        }
                        .chars()
                    })
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => TileWide::Wall,
                        '.' => TileWide::Free,
                        '[' => TileWide::BoxLeft,
                        ']' => TileWide::BoxRight,
                        '@' => {
                            robot_x = Some(x);
                            robot_y = Some(y);
                            TileWide::Free
                        }
                        _ => panic!("Unexpected tile"),
                    })
                    .collect(),
            );
        }

        Self {
            map,
            robot_x: robot_x.expect("Robot X"),
            robot_y: robot_y.expect("Rboot Y"),
        }
    }

    fn move_left(&mut self) {
        if self.attemp_left(self.robot_y, self.robot_x - 1) {
            self.robot_x -= 1;
        }
    }

    fn attemp_left(&mut self, target_y: usize, target_x: usize) -> bool {
        match self.get_at(target_y, target_x) {
            TileWide::Free => true,
            TileWide::Wall => false,
            t if self.attemp_left(target_y, target_x - 1) => {
                self.map[target_y][target_x - 1] = t;
                self.map[target_y][target_x] = TileWide::Free;
                true
            }
            _ => false,
        }
    }

    fn get_at(&self, target_y: usize, target_x: usize) -> TileWide {
        self.map[target_y][target_x]
    }

    fn move_right(&mut self) {
        if self.attemp_right(self.robot_y, self.robot_x + 1) {
            self.robot_x += 1;
        }
    }

    fn attemp_right(&mut self, target_y: usize, target_x: usize) -> bool {
        match self.get_at(target_y, target_x) {
            TileWide::Free => true,
            TileWide::Wall => false,
            t if self.attemp_right(target_y, target_x + 1) => {
                self.map[target_y][target_x + 1] = t;
                self.map[target_y][target_x] = TileWide::Free;
                true
            }
            _ => false,
        }
    }

    fn move_up(&mut self) {
        if self.can_move_up(self.robot_y - 1, self.robot_x) {
            self.do_move_up(self.robot_y - 1, self.robot_x);
            self.robot_y -= 1;
        }
    }

    fn can_move_up(&self, target_y: usize, target_x: usize) -> bool {
        match self.get_at(target_y, target_x) {
            TileWide::Free => true,
            TileWide::Wall => false,
            TileWide::BoxLeft => {
                self.can_move_up(target_y - 1, target_x)
                    && self.can_move_up(target_y - 1, target_x + 1)
            }
            TileWide::BoxRight => {
                self.can_move_up(target_y - 1, target_x)
                    && self.can_move_up(target_y - 1, target_x - 1)
            }
        }
    }

    fn do_move_up(&mut self, target_y: usize, target_x: usize) {
        match self.get_at(target_y, target_x) {
            TileWide::BoxLeft => {
                self.do_move_up(target_y - 1, target_x);
                self.do_move_up(target_y - 1, target_x + 1);
                self.map[target_y - 1][target_x] = TileWide::BoxLeft;
                self.map[target_y - 1][target_x + 1] = TileWide::BoxRight;
                self.map[target_y][target_x] = TileWide::Free;
                self.map[target_y][target_x + 1] = TileWide::Free;
            }
            TileWide::BoxRight => {
                self.do_move_up(target_y - 1, target_x);
                self.do_move_up(target_y - 1, target_x - 1);
                self.map[target_y - 1][target_x - 1] = TileWide::BoxLeft;
                self.map[target_y - 1][target_x] = TileWide::BoxRight;
                self.map[target_y][target_x] = TileWide::Free;
                self.map[target_y][target_x - 1] = TileWide::Free;
            }
            _ => (),
        }
    }

    fn move_down(&mut self) {
        if self.can_move_down(self.robot_y + 1, self.robot_x) {
            self.do_move_down(self.robot_y + 1, self.robot_x);
            self.robot_y += 1;
        }
    }

    fn can_move_down(&self, target_y: usize, target_x: usize) -> bool {
        match self.get_at(target_y, target_x) {
            TileWide::Free => true,
            TileWide::Wall => false,
            TileWide::BoxLeft => {
                self.can_move_down(target_y + 1, target_x)
                    && self.can_move_down(target_y + 1, target_x + 1)
            }
            TileWide::BoxRight => {
                self.can_move_down(target_y + 1, target_x)
                    && self.can_move_down(target_y + 1, target_x - 1)
            }
        }
    }

    fn do_move_down(&mut self, target_y: usize, target_x: usize) {
        match self.get_at(target_y, target_x) {
            TileWide::BoxLeft => {
                self.do_move_down(target_y + 1, target_x);
                self.do_move_down(target_y + 1, target_x + 1);
                self.map[target_y][target_x] = TileWide::Free;
                self.map[target_y][target_x + 1] = TileWide::Free;
                self.map[target_y + 1][target_x] = TileWide::BoxLeft;
                self.map[target_y + 1][target_x + 1] = TileWide::BoxRight;
            }
            TileWide::BoxRight => {
                self.do_move_down(target_y + 1, target_x);
                self.do_move_down(target_y + 1, target_x - 1);
                self.map[target_y][target_x] = TileWide::Free;
                self.map[target_y][target_x - 1] = TileWide::Free;
                self.map[target_y + 1][target_x] = TileWide::BoxRight;
                self.map[target_y + 1][target_x - 1] = TileWide::BoxLeft;
            }
            _ => (),
        }
    }

    fn sum_gps(&self) -> usize {
        let mut result = 0;

        self.map.iter().enumerate().for_each(|(row_index, row)| {
            row.iter().enumerate().for_each(|(col_index, tile)| {
                if *tile == TileWide::BoxLeft {
                    result += row_index * 100 + col_index;
                }
            });
        });

        result
    }

    fn dump_state(&self) {
        self.map.iter().enumerate().for_each(|(r_index, r)| {
            println!(
                "{}",
                r.iter()
                    .enumerate()
                    .map(|(t_index, t)| {
                        if r_index == self.robot_y && t_index == self.robot_x {
                            return '@';
                        }
                        match t {
                            TileWide::BoxLeft => '[',
                            TileWide::BoxRight => ']',
                            TileWide::Free => '.',
                            TileWide::Wall => '#',
                        }
                    })
                    .collect::<String>()
            );
        });
    }

    fn validate_state(&self) {
        let mut expected = None;
        self.map.iter().for_each(|r| {
            r.iter().for_each(|t| match expected {
                Some(e) => {
                    assert_eq!(e, *t);
                    expected = None;
                }
                None => match t {
                    TileWide::BoxLeft => expected = Some(TileWide::BoxRight),
                    TileWide::BoxRight => panic!("Unexpected right box"),
                    _ => (),
                },
            });
        });
    }
}
