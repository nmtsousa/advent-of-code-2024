use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(width: i16, height: i16, reader: R) -> Result<usize> {
        let mut bathroom = Bathroom::new(width, height);

        let re = Regex::new(r"^p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)$")?;

        reader.lines().map_while(Result::ok).for_each(|l| {
            let captures = re.captures(&l).expect("Button A captured.");

            let p_x = captures[1].parse::<i16>().expect("Number for X");
            let p_y = captures[2].parse::<i16>().expect("Number for Y");

            let v_x = captures[3].parse::<i16>().expect("Number for X");
            let v_y = captures[4].parse::<i16>().expect("Number for Y");

            bathroom.add_robot(p_x, p_y, v_x, v_y);
        });

        bathroom.tick(100);

        Ok(bathroom.safety_factor())
    }

    assert_eq!(12, part1(11, 7, BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(101, 103, input_file)?);
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
struct Bathroom {
    width: i16,
    height: i16,
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    p_x: i16,
    p_y: i16,
    v_x: i16,
    v_y: i16,
}

impl Bathroom {
    fn new(width: i16, height: i16) -> Bathroom {
        Bathroom {
            width,
            height,
            robots: vec![],
        }
    }

    fn add_robot(&mut self, p_x: i16, p_y: i16, v_x: i16, v_y: i16) {
        self.robots.push(Robot { p_x, p_y, v_x, v_y });
    }

    fn tick(&mut self, count: usize) {
        for _ in 0..count {
            for robot in self.robots.iter_mut() {
                robot.tick(self.width, self.height);
            }
        }
    }

    fn safety_factor(&self) -> usize {
        let mid_width = self.width / 2;
        let mid_height = self.height / 2;

        let mut q1: usize = 0;
        let mut q2: usize = 0;
        let mut q3: usize = 0;
        let mut q4: usize = 0;

        for robot in self.robots.iter() {
            if robot.p_y < mid_height {
                if robot.p_x < mid_width {
                    q1 += 1;
                }
                if robot.p_x > mid_width {
                    q2 += 1;
                }
            }
            if robot.p_y > mid_height {
                if robot.p_x < mid_width {
                    q3 += 1;
                }
                if robot.p_x > mid_width {
                    q4 += 1;
                }
            }
        }

        q1 * q2 * q3 * q4
    }
}

impl Robot {
    fn tick(&mut self, width: i16, height: i16) {
        self.p_x += self.v_x;
        self.p_y += self.v_y;
        self.p_x %= width;
        self.p_y %= height;
        if self.p_x < 0 {
            self.p_x += width;
        }
        if self.p_y < 0 {
            self.p_y += height;
        }
    }
}
