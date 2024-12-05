use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, fs};

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}
fn parse_ordering_rule(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_number, tag("|"), parse_number)(input)
}
fn parse_ordering_rules(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(line_ending, parse_ordering_rule)(input)
}

fn parse_pages(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, separated_list1(tag(","), parse_number))(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    separated_pair(parse_ordering_rules, tag("\n\n"), parse_pages)(input)
}

fn solve_part_one((rules, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut rules_map = HashMap::new();

    rules.iter().for_each(|(a, b)| {
        rules_map.entry(a).or_insert(Vec::new()).push(b);
    });

    // for each update. iterate thru it backwards. Collect all the preceding pages. If you see one of these pages it's bad!
    let okay_updates = updates.iter().filter(|update| {
        let mut preceding_pages: Vec<&i32> = Vec::new();
        for page in update.iter().rev() {
            if let Some(preceding) = rules_map.get(page) {
                preceding_pages.append(&mut preceding.clone());
            }
            if preceding_pages.contains(&page) {
                return false;
            }
        }
        true
    });

    okay_updates.fold(0, |acc, update| acc + update[update.len() / 2])
}

fn solve_part_two((rules, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut rules_map = HashMap::new();

    rules.iter().for_each(|(a, b)| {
        rules_map.entry(a).or_insert(Vec::new()).push(b);
    });

    // for each update. iterate thru it backwards. Collect all the preceding pages. If you see one of these pages it's bad!
    let incorrect_updates = updates.iter().filter(|update| {
        let mut preceding_pages: Vec<&i32> = Vec::new();
        for page in update.iter().rev() {
            if let Some(preceding) = rules_map.get(page) {
                preceding_pages.append(&mut preceding.clone());
            }
            if preceding_pages.contains(&page) {
                return true;
            }
        }
        false
    });

    incorrect_updates.fold(0, |acc, update| {
        let mut update = update.clone();
        let mut sorted = false;
        while !sorted {
            sorted = true;

            let mut swap_indices = None;
            let mut preceding_pages: Vec<&i32> = Vec::new();
            for (index, page) in update.iter().enumerate().rev() {
                if let Some(preceding) = rules_map.get(page) {
                    preceding_pages.append(&mut preceding.clone());
                }

                if preceding_pages.contains(&page) {
                    swap_indices = Some(index);
                    sorted = false; // Found a swap needed, so not sorted
                    break;
                }
            }

            if let Some(ix) = swap_indices {
                update.swap(ix, ix + 1);
            }
        }

        acc + update[update.len() / 2]
    })
}

fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    let res = parse_input(&input).expect("failed to parse");
    println!("part one {:?}", solve_part_one(&res.1));
    println!("part two {:?}", solve_part_two(&res.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_input() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let result = parse_input(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            ((
                vec![
                    (47, 53),
                    (97, 13),
                    (97, 61),
                    (97, 47),
                    (75, 29),
                    (61, 13),
                    (75, 53),
                    (29, 13),
                    (97, 29),
                    (53, 29),
                    (61, 53),
                    (97, 53),
                    (61, 29),
                    (47, 13),
                    (75, 47),
                    (97, 75),
                    (47, 61),
                    (75, 61),
                    (47, 29),
                    (75, 13),
                    (53, 13)
                ],
                vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13,],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53,],
                    vec![61, 13, 29,],
                    vec![97, 13, 75, 29, 47]
                ]
            ))
        )
    }

    #[test]
    fn test_part_one() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let parsed = parse_input(input).expect("failed to parse");
        let solution = solve_part_one(&parsed.1);
        assert_eq!(solution, 143)
    }

    #[test]
    fn test_part_two() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let parsed = parse_input(input).expect("failed to parse");
        let solution = solve_part_two(&parsed.1);
        assert_eq!(solution, 123)
    }
}
