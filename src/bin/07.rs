use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Sum,
    Multiply,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation2 {
    Sum,
    Multiply,
    Concatenate,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut sum_of_test_values: usize = 0;
        for line in reader.lines().map_while(Result::ok) {
            let mut line_split = line.split_whitespace();
            let total = line_split
                .next()
                .expect("Expected first number in line")
                .split(":")
                .next()
                .expect("Number before :")
                .parse::<usize>()
                .unwrap();

            let parts: Vec<usize> = line_split.flat_map(|n| n.parse::<usize>()).collect();
            let mut operations = vec![Operation::Sum; parts.len() - 1];
            let mut missing_mutations = true;
            while missing_mutations {
                let mut result = 0;

                match operations[0] {
                    Operation::Sum => result += parts[0] + parts[1],
                    Operation::Multiply => result += parts[0] * parts[1],
                }

                for i in 1..operations.len() {
                    match operations[i] {
                        Operation::Sum => result += parts[i + 1],
                        Operation::Multiply => result *= parts[i + 1],
                    }
                }

                if result == total {
                    sum_of_test_values += total;
                    break;
                }

                missing_mutations = mutate_operations(&mut operations);
            }
        }
        Ok(sum_of_test_values)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut sum_of_test_values: usize = 0;
        for line in reader.lines().map_while(Result::ok) {
            let mut line_split = line.split_whitespace();
            let total = line_split
                .next()
                .expect("Expected first number in line")
                .split(":")
                .next()
                .expect("Number before :")
                .parse::<usize>()
                .unwrap();

            let parts: Vec<usize> = line_split.flat_map(|n| n.parse::<usize>()).collect();
            let mut operations = vec![Operation2::Sum; parts.len() - 1];
            let mut missing_mutations = true;
            while missing_mutations {
                let mut result;

                match operations[0] {
                    Operation2::Sum => result = parts[0] + parts[1],
                    Operation2::Multiply => result = parts[0] * parts[1],
                    Operation2::Concatenate => result = concatenate(parts[0], parts[1]),
                }

                for i in 1..operations.len() {
                    match operations[i] {
                        Operation2::Sum => result += parts[i + 1],
                        Operation2::Multiply => result *= parts[i + 1],
                        Operation2::Concatenate => result = concatenate(result, parts[i + 1]),
                    }
                }
                if result == total {
                    sum_of_test_values += total;
                    break;
                }

                missing_mutations = mutate_operations2(&mut operations);
            }
        }
        Ok(sum_of_test_values)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn concatenate(a: usize, b: usize) -> usize {
    let astr = a.to_string();
    let bstr = b.to_string();

    let mut result: String = astr.clone();
    result.push_str(&bstr);

    result.parse::<usize>().expect("Number is parsable")
}

fn mutate_operations(operations: &mut [Operation]) -> bool {
    if all_are(operations, Operation::Multiply) {
        return false;
    }

    match operations[0] {
        Operation::Sum => operations[0] = Operation::Multiply,
        Operation::Multiply => {
            operations[0] = Operation::Sum;
            for i in 1..operations.len() {
                match operations[i] {
                    Operation::Sum => {
                        operations[i] = Operation::Multiply;
                        break;
                    }
                    Operation::Multiply => operations[i] = Operation::Sum,
                }
            }
        }
    }
    true
}

fn mutate_operations2(operations: &mut [Operation2]) -> bool {
    if all_are2(operations, Operation2::Concatenate) {
        return false;
    }

    match operations[0] {
        Operation2::Sum => operations[0] = Operation2::Multiply,
        Operation2::Multiply => operations[0] = Operation2::Concatenate,
        Operation2::Concatenate => {
            operations[0] = Operation2::Sum;
            for i in 1..operations.len() {
                match operations[i] {
                    Operation2::Sum => {
                        operations[i] = Operation2::Multiply;
                        break;
                    }
                    Operation2::Multiply => {
                        operations[i] = Operation2::Concatenate;
                        break;
                    }
                    Operation2::Concatenate => operations[i] = Operation2::Sum,
                }
            }
        }
    }
    true
}

fn all_are(operations: &[Operation], operation: Operation) -> bool {
    for op in operations {
        if *op != operation {
            return false;
        }
    }

    true
}

fn all_are2(operations: &[Operation2], operation: Operation2) -> bool {
    for op in operations {
        if *op != operation {
            return false;
        }
    }

    true
}
