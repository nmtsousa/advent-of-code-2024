use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut garden = Garden::new(reader.lines().map_while(Result::ok));
        Ok(garden.compute())
    }

    assert_eq!(140, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(1930, part1(BufReader::new(TEST3.as_bytes()))?);

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

    fn compute(&mut self) -> usize {
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                match self.map[row][col].region {
                    Some(_) => continue,
                    None => {
                        self.map_to_regtion(self.reg_count, row, col);
                        self.reg_count += 1;
                    }
                };
            }
        }

        let mut result: usize = 0;
        for (key, area) in self.area_map.iter() {
            let perimeter = self.perimeter_map.get(&key).expect("Area has perimter");
            result += *area * *perimeter;
        }

        result
    }

    fn map_to_regtion(&mut self, region: usize, row: usize, col: usize) {
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
            self.map_to_regtion(region, row - 1, col);
        } else {
            match self.perimeter_map.get(&region) {
                None => self.perimeter_map.insert(region, 1),
                Some(p) => self.perimeter_map.insert(region, p + 1),
            };
        }

        if row + 1 < self.row_count
            && self.map[row + 1][col].plant_type == self.map[row][col].plant_type
        {
            self.map_to_regtion(region, row + 1, col);
        } else {
            match self.perimeter_map.get(&region) {
                None => self.perimeter_map.insert(region, 1),
                Some(p) => self.perimeter_map.insert(region, p + 1),
            };
        }

        if col > 0 && self.map[row][col - 1].plant_type == self.map[row][col].plant_type {
            self.map_to_regtion(region, row, col - 1);
        } else {
            match self.perimeter_map.get(&region) {
                None => self.perimeter_map.insert(region, 1),
                Some(p) => self.perimeter_map.insert(region, p + 1),
            };
        }

        if col + 1 < self.col_count
            && self.map[row][col + 1].plant_type == self.map[row][col].plant_type
        {
            self.map_to_regtion(region, row, col + 1);
        } else {
            match self.perimeter_map.get(&region) {
                None => self.perimeter_map.insert(region, 1),
                Some(p) => self.perimeter_map.insert(region, p + 1),
            };
        }
    }
}
