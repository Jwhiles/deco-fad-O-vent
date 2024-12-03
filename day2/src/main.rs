use nom::{
    character::complete::{digit1, space1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::fs;

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, parse_number)(input)
}
fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, parse_numbers)(input)
}


fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    let (_, res) = parse_lines(&input).expect("failed to parse");

    let part_one = res.iter().fold(0, |acc, line| {
        if test_list(line) {
            acc + 1
        } else {
            acc
        }
    });

    let part_two = res.iter().fold(0, |acc, line| {
        match check(line) {
            Some(_) => acc + 1,
            None => acc,
        }
    });

    println!("result part one: {:?}", part_one);
    println!("result part two: {:?}", part_two);
}


fn check (input: &[i32]) -> Option<i32>{
    if test_list(input) {
      return Some(1)
    }

    for (i, _) in input.iter().enumerate() {
        let one_item_removed: Vec<_> = [&input[..i], &input[i + 1..]].concat();
        if test_list(&one_item_removed) {
            return Some(1)
        }
    }

    None
}

fn test_list (input: &[i32]) -> bool {
    let changes = input.windows(2).map(|window| {
        return window[0] - window[1]
    }).collect::<Vec<i32>>();

    return test_signums(&changes) && test_diff(&changes) 
}

fn test_signums (input: &[i32]) -> bool {
    let signum = input[0].signum();

    input.iter().all(|&x| x.signum() == signum)
}

fn test_diff (input: &[i32]) -> bool {
    input.iter().all(|&x| x.abs() <= 3 && x.abs() > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_safe() {
        let input: Vec<i32> = vec![7, 6, 4, 2, 1];
        let result = check(&input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn always_unsafe() {
        let input = vec![1, 2, 7, 8, 9];
        let result = check(&input);
        assert!(result.is_none());
    }

    #[test]
    fn safe_with_removals() {
        let input = vec![1, 3, 2, 4, 5];
        let result = check(&input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 1);
    }
}
