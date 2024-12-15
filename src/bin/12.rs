use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
AAAA
BBCD
BBCC
EEEC
";

const TEST2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

const TEST3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const TEST4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

const TEST5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut garden = Garden::new(reader.lines().map_while(Result::ok));
        Ok(garden.compute_with_perimeter())
    }

    assert_eq!(140, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(1930, part1(BufReader::new(TEST3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut garden = Garden::new(reader.lines().map_while(Result::ok));
        Ok(garden.compute_with_sides())
    }

    assert_eq!(80, part2(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(436, part2(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(236, part2(BufReader::new(TEST4.as_bytes()))?);
    assert_eq!(368, part2(BufReader::new(TEST5.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug)]
struct Garden {
    map: Vec<Vec<Plot>>,
    row_count: usize,
    col_count: usize,
    reg_count: usize,
    area_map: HashMap<usize, usize>,
    perimeter_map: HashMap<usize, usize>,
}

#[derive(Debug)]
struct Plot {
    plant_type: char,
    region: Option<usize>,
}

impl Garden {
    fn new(lines: impl Iterator<Item = String>) -> Self {
        let mut map = vec![];
        let mut row_count = 0;
        let mut col_count = 0;

        for line in lines {
            let row: Vec<Plot> = line
                .chars()
                .map(|c| Plot {
                    plant_type: c,
                    region: None,
                })
                .collect();
            row_count += 1;
            if col_count == 0 {
                col_count = row.len();
            }

            map.push(row);
        }

        Self {
            map,
            row_count,
            col_count,
            reg_count: 0,
            area_map: HashMap::new(),
            perimeter_map: HashMap::new(),
        }
    }

    fn compute_with_perimeter(&mut self) -> usize {
        self.compute_regions();

        let mut result: usize = 0;
        for (key, area) in self.area_map.iter() {
            let perimeter = self.perimeter_map.get(&key).expect("Area has perimter");
            result += *area * *perimeter;
        }

        result
    }

    fn compute_regions(&mut self) {
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                match self.map[row][col].region {
                    Some(_) => continue,
                    None => {
                        self.map_to_region(self.reg_count, row, col);
                        self.reg_count += 1;
                    }
                };
            }
        }
    }

    fn map_to_region(&mut self, region: usize, row: usize, col: usize) {
        match self.map[row][col].region {
            Some(r) => {
                assert!(r == region);
                return;
            }
            None => self.map[row][col].region = Some(region),
        };

        match self.area_map.get(&region) {
            None => self.area_map.insert(region, 1),
            Some(a) => self.area_map.insert(region, a + 1),
        };

        if row > 0 && self.map[row - 1][col].plant_type == self.map[row][col].plant_type {
            self.map_to_region(region, row - 1, col);
        } else {
            match self.perimeter_map.get(&region) {
                None => self.perimeter_map.insert(region, 1),
                Some(p) => self.perimeter_map.insert(region, p + 1),
            };
        }

        if row + 1 < self.row_count
            && self.map[row + 1][col].plant_type == self.map[row][col].plant_type
        {
            self.map_to_region(region, row + 1, col);
        } else {
            match self.perimeter_map.get(&region) {
                None => self.perimeter_map.insert(region, 1),
                Some(p) => self.perimeter_map.insert(region, p + 1),
            };
        }

        if col > 0 && self.map[row][col - 1].plant_type == self.map[row][col].plant_type {
            self.map_to_region(region, row, col - 1);
        } else {
            match self.perimeter_map.get(&region) {
                None => self.perimeter_map.insert(region, 1),
                Some(p) => self.perimeter_map.insert(region, p + 1),
            };
        }

        if col + 1 < self.col_count
            && self.map[row][col + 1].plant_type == self.map[row][col].plant_type
        {
            self.map_to_region(region, row, col + 1);
        } else {
            match self.perimeter_map.get(&region) {
                None => self.perimeter_map.insert(region, 1),
                Some(p) => self.perimeter_map.insert(region, p + 1),
            };
        }
    }

    fn compute_with_sides(&mut self) -> usize {
        self.compute_with_perimeter();

        let mut result: usize = 0;
        for (key, area) in self.area_map.iter() {
            let side_count = self.count_sides(*key);
            result += *area * side_count;
        }

        result
    }

    fn count_sides(&self, region: usize) -> usize {
        let mut horizontal_sides = HashMap::new();
        let mut vertical_sides = HashMap::new();
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                if self.map[row][col].region != Some(region) {
                    continue;
                }

                if col == 0 || self.map[row][col - 1].region != Some(region) {
                    vertical_sides
                        .entry(col)
                        .or_insert(Vec::new())
                        .push(Wall::new(row, Side::Left));
                }

                if col + 1 == self.col_count || self.map[row][col + 1].region != Some(region) {
                    vertical_sides
                        .entry(col + 1)
                        .or_insert(Vec::new())
                        .push(Wall::new(row, Side::Right));
                }

                if row == 0 || self.map[row - 1][col].region != Some(region) {
                    horizontal_sides
                        .entry(row)
                        .or_insert(Vec::new())
                        .push(Wall::new(col, Side::Top));
                }

                if row + 1 == self.col_count || self.map[row + 1][col].region != Some(region) {
                    horizontal_sides
                        .entry(row + 1)
                        .or_insert(Vec::new())
                        .push(Wall::new(col, Side::Bottom));
                }
            }
        }

        let count = self.compute_sides(horizontal_sides) + self.compute_sides(vertical_sides);

        count
    }

    fn compute_sides(&self, wall_map: HashMap<usize, Vec<Wall>>) -> usize {
        let mut count = 0;

        for walls in wall_map.values() {
            let mut last_wall = None;
            for wall in walls {
                match last_wall {
                    None => {
                        last_wall = Some(wall);
                        count += 1;
                    }
                    Some(c) => {
                        if c.index + 1 == wall.index && c.side == wall.side {
                            last_wall = Some(wall);
                        } else {
                            last_wall = Some(wall);
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}

#[derive(Debug)]
struct Wall {
    index: usize,
    side: Side,
}

#[derive(Debug, PartialEq, Eq)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Wall {
    fn new(index: usize, side: Side) -> Wall {
        Wall { index, side }
    }
}
