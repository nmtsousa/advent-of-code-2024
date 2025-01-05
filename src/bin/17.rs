use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const EXAMPLE_1: &str = "\
Register A: 0
Register B: 0
Register C: 9

Program: 2,6
";

const EXAMPLE_2: &str = "\
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
";

const EXAMPLE_3: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const EXAMPLE_4: &str = "\
Register A: 0
Register B: 29
Register C: 0

Program: 1,7
";

const EXAMPLE_5: &str = "\
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
";

const TEST_1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

#[derive(Debug)]
struct Computer {
    ins_ptr: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn new(lines: &mut impl Iterator<Item = String>) -> Self {
        let reg_a = parse_reg(lines);
        let reg_b = parse_reg(lines);
        let reg_c = parse_reg(lines);

        lines.next().expect("Empty line");

        let program = lines
            .next()
            .expect("Program line")
            .split(": ")
            .collect_vec()[1]
            .split(",")
            .flat_map(|s| s.parse::<u8>())
            .collect_vec();

        Self {
            ins_ptr: 0,
            reg_a,
            reg_b,
            reg_c,
            program,
            output: vec![],
        }
    }
    fn get_output(&self) -> String {
        self.output.iter().join(",")
    }

    fn execute(&mut self) {
        while self.ins_ptr < self.program.len() {
            let opcode = self.program[self.ins_ptr];
            let operand = self.program[self.ins_ptr + 1];

            match opcode {
                // adv
                0 => {
                    let combo = self.combo(operand);
                    let base: usize = 2;
                    let result = self.reg_a / base.pow(combo.try_into().unwrap());
                    self.reg_a = result;
                },

                // bst
                2 => {
                    let combo = self.combo(operand);
                    let result = combo % 8;
                    self.reg_b = result;
                }

                // jnz
                3 => {
                    if self.reg_a != 0 {
                        self.ins_ptr = operand.into();
                        continue;
                    }
                },

                // out
                5 => {
                    let combo = self.combo(operand);
                    let result = combo % 8;
                    self.output.push(result.try_into().unwrap());
                }

                x => todo!("Opcode {} not implemented.", x),
            }
            self.ins_ptr += 2;
        }
    }

    fn combo(&self, operand: u8) -> usize {
        match operand {
            x if x < 4 => x.into(),
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Combo operand {} not expected", operand),
        }
    }
}

fn parse_reg(lines: &mut impl Iterator<Item = String>) -> usize {
    lines.next().expect("Reg line").split(": ").collect_vec()[1]
        .parse::<usize>()
        .expect("I can parse the register.")
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<Computer> {
        let mut lines = reader.lines().map_while(Result::ok);
        let mut comp = Computer::new(&mut lines);
        comp.execute();
        Ok(comp)
    }

    assert_eq!(1, part1(BufReader::new(EXAMPLE_1.as_bytes()))?.reg_b);

    assert_eq!(
        "0,1,2",
        part1(BufReader::new(EXAMPLE_2.as_bytes()))?.get_output()
    );

    {
        let test3 = part1(BufReader::new(EXAMPLE_3.as_bytes()))?;
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", test3.get_output());
        assert_eq!(0, test3.reg_a);
    }

    assert_eq!(7, part1(BufReader::new(EXAMPLE_4.as_bytes()))?.reg_b);

    assert_eq!(44354, part1(BufReader::new(EXAMPLE_5.as_bytes()))?.reg_b);

    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        part1(BufReader::new(TEST_1.as_bytes()))?.get_output()
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result.get_output());
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
