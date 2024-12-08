use std::fs;

use nom::{
    bytes::complete::tag,
    character::{
        complete::line_ending,
        complete::{digit1, space1},
    },
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

fn parse_line(input: &str) -> IResult<&str, (i64, Vec<i64>)> {
    separated_pair(
        parse_number,
        tag(": "),
        separated_list1(space1, parse_number),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i64, Vec<i64>)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn solve_part_one(input: &[(i64, Vec<i64>)]) -> i64 {
    let operations = [|a, b| a * b, |a, b| a + b];

    input.iter().fold(0, |acc, (result, inputs)| {
        if test_line_both(&operations, result, inputs, 0) {
            return acc + result;
        }
        return acc;
    })
}

fn combine_numbers(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn test_line_both<'a>(
    operations: &[fn(i64, i64) -> i64],
    target: &i64,
    inputs: &'a [i64],
    acc: i64,
) -> bool {
    if inputs.is_empty() {
        return acc == *target;
    }

    let mut new_acc = acc;
    let mut next = inputs[0];

    if new_acc == 0 {
        new_acc = inputs[0];
        next = inputs[1];
    }
    let remaining = &inputs[if acc == 0 { 2 } else { 1 }..];

    let mut result = false;

    for operation in operations.iter() {
        let op_res = operation(new_acc, next);
        if op_res <= *target {
            result = result || test_line_both(operations, target, remaining, op_res);
        }
    }

    result
}

fn solve_part_two(input: &[(i64, Vec<i64>)]) -> i64 {
    let operations = [|a, b| a * b, |a, b| a + b, combine_numbers];

    input.iter().fold(0, |acc, (result, inputs)| {
        if test_line_both(&operations, result, inputs, 0) {
            return acc + result;
        }
        return acc;
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
    fn test_parse_line() {
        let input = "190: 10 19";
        let result = parse_line(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().1, (190, vec![10, 19]));
    }

    const FULL_INPUT: &str = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

    #[test]
    fn parse_full_input() {
        let result = parse_input(FULL_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().1.len(), 9);
    }

    #[test]
    fn test_solve_part_one() {
        let result = parse_input(FULL_INPUT).expect("failed to parse");
        let solution = solve_part_one(&result.1);
        assert_eq!(solution, 3749);
    }

    #[test]
    fn test_solve_part_two() {
        let result = parse_input(FULL_INPUT).expect("failed to parse");
        let solution = solve_part_two(&result.1);
        assert_eq!(solution, 11387);
    }
}
