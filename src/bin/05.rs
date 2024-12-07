use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::convert::From;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

struct Rule {
    page: String,
    before: String,
}

impl Rule {
    fn new(page: String, before: String) -> Rule {
        Rule { page, before }
    }
}

impl From<String> for Rule {
    fn from(rule: String) -> Self {
        let parts: Vec<&str> = rule.split("|").collect();

        Rule::new(
            parts.first().unwrap().to_string(),
            parts.get(1).unwrap().to_string(),
        )
    }
}

struct RuleList {
    list: Vec<Rule>,
}

impl RuleList {
    fn new() -> RuleList {
        RuleList { list: vec![] }
    }

    fn add(&mut self, rule: Rule) {
        self.list.push(rule);
    }

    fn is_valid(&self, update: &[String]) -> bool {
        for rule in self.list.iter() {
            let page_index = update.iter().position(|r| *r == rule.page);
            let before_index = update.iter().position(|r| *r == rule.before);

            if let Some(pi) = page_index {
                if let Some(bi) = before_index {
                    if pi > bi {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut rule_list = RuleList::new();
        let mut load = true;
        let mut result = 0;

        for line in reader.lines() {
            match line {
                Result::Ok(read) if read.is_empty() => load = false,
                Result::Ok(read) => {
                    if load {
                        rule_list.add(Rule::from(read));
                    } else {
                        let update: Vec<String> =
                            read.split(",").map(|str| str.to_string()).collect();
                        if rule_list.is_valid(&update) {
                            let middle = update.len() / 2;
                            let value = update[middle].parse::<usize>().unwrap();
                            result += value;
                        }
                    }
                }
                Err(_) => panic!("Error reading"),
            }
        }

        Ok(result)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

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
