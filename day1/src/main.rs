use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::fs;

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_two_numbers(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_number, space1, parse_number)(input)
}

fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");

    let (mut left, mut right) = input.lines().fold(
        (vec![], vec![]),
        |(mut left, mut right), line| match parse_two_numbers(line) {
            Ok((_, (a, b))) => {
                left.push(a);
                right.push(b);
                (left, right)
            }
            Err(_) => (left, right),
        },
    );

    left.sort();
    right.sort();

    println!("{}", solve_part_one(&left, &right));
    println!("{:?}", solve_part_two(&left, &right));
}

fn solve_part_one(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs())
}

fn solve_part_two(left: &[i32], right: &[i32]) -> i32 {
    let frequencies = right.iter().fold(HashMap::new(), |mut map, &num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    left.iter().fold(0, |acc, num| {
        acc + (frequencies.get(num).unwrap_or(&0) * num)
    })
}
