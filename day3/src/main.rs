use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::fs;

#[derive(Debug)]
enum Operation {
    Do,
    Dont,
    Mul(i32, i32),
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_mul(input: &str) -> IResult<&str, Option<Operation>> {
    let res = preceded(
        tag("mul"),
        opt(delimited(
            char('('),
            separated_pair(parse_number, char(','), parse_number),
            char(')'),
        )),
    )(input);

    res.map(|(input, res)| (input, res.map(|(a, b)| Operation::Mul(a, b))))
}

fn parse_until_operation(input: &str) -> IResult<&str, &str> {
    let operations = ["mul", "do()", "don't()"];
    let mut earliest: Option<(usize, &str)> = None;

    // Find the earliest occurring operation
    for &op in operations.iter() {
        if let Some(pos) = input.find(op) {
            match earliest {
                None => earliest = Some((pos, op)),
                Some((earliest_pos, _)) if pos < earliest_pos => earliest = Some((pos, op)),
                _ => {}
            }
        }
    }

    match earliest {
        Some((_, op)) => {
            let (input, _) = opt(take_until(op))(input)?;
            Ok((input, op))
        }
        None => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

fn parse_do(input: &str) -> IResult<&str, Option<Operation>> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Some(Operation::Do)))
}

fn parse_dont(input: &str) -> IResult<&str, Option<Operation>> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Some(Operation::Dont)))
}

fn parse_operation(input: &str) -> IResult<&str, Option<Operation>> {
    let (input, op) = parse_until_operation(input)?;
    match op {
        "do()" => parse_do(input),
        "don't()" => parse_dont(input),
        "mul" => parse_mul(input),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

fn parse_input(input: &str) -> Option<Vec<Operation>> {
    match many0(parse_operation)(input) {
        Ok((_, res)) => Some(res.into_iter().filter_map(|x| x).collect()),
        _ => None,
    }
}

fn solve_part_one(input: &[Operation]) -> i32 {
    input.iter().fold(0, |acc, op| match op {
        Operation::Mul(a, b) => acc + a * b,
        _ => acc,
    })
}

fn solve_part_two(input: &[Operation]) -> i32 {
    let mut mul_enabled = true;
    input.iter().fold(0, |acc, op| match op {
        Operation::Mul(a, b) => {
            if mul_enabled {
                acc + a * b
            } else {
                acc
            }
        }
        Operation::Do => {
            mul_enabled = true;
            acc
        }
        Operation::Dont => {
            mul_enabled = false;
            acc
        }
    })
}

fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    let res = parse_input(&input).expect("failed to parse");
    println!("part one {:?}", solve_part_one(&res));
    println!("part two {:?}", solve_part_two(&res));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_many_mul() {
        let input = "mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = parse_input(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 4);
    }

    #[test]
    fn part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed = parse_input(input).expect("failed to parse");
        assert_eq!(solve_part_one(&parsed), 161);
    }

    #[test]
    fn part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let parsed = parse_input(input).expect("failed to parse");
        assert_eq!(solve_part_two(&parsed), 48);
    }
}
