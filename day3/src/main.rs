use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::{
        complete::{char, digit1},
        streaming::{alpha0, alpha1},
    },
    combinator::{map, map_res, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::fs;

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_brax(input: &str) -> IResult<&str, Option<(i32, i32)>> {
    let res = opt(delimited(
        char('('),
        separated_pair(parse_number, char(','), parse_number),
        char(')'),
    ))(input);

    res
}

fn parse_until_mul(input: &str) -> IResult<&str, &str> {
    let (input, res) = opt(take_until("mul"))(input)?;
    match res {
        Some(_) => tag("mul")(input),
        None => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

fn parse_input(input: &str) -> Option<Vec<(i32, i32)>> {
    match many0(preceded(parse_until_mul, parse_brax))(input) {
        Ok((_, res)) => Some(res.into_iter().filter_map(|x| x).collect()),
        _ => None,
    }
}

fn solve_part_one(input: &[(i32, i32)]) -> i32 {
    input.iter().fold(0, |acc, (a, b)| acc + a * b)
}

fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    let res = parse_input(&input).expect("failed to parse");
    println!("part one {:?}", solve_part_one(&res));
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed = parse_input(input).expect("failed to parse");
        assert_eq!(solve_part_two(&parsed), 48);
    }
}
