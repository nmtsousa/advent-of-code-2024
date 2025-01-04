use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[derive(Debug, Default)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    outuput: Vec<u8>,
}

impl Computer {
    fn get_output(&self) -> String {
        todo!("To implement");
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<Computer> {
        let comp = Computer::default();
        // TODO: Solve Part 1 of the puzzle
        let answer = reader.lines().flatten().count();
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
