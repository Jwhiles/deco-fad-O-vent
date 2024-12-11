use std::{fs, iter::repeat};

use nom::{
    character::complete::{anychar, multispace0},
    combinator::{eof, map, map_res},
    multi::many_till,
    sequence::preceded,
    IResult,
};

const RADIX: u32 = 10;
fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(anychar, |c| {
        c.to_digit(RADIX).map(|d| d as i64).ok_or("Invalid digit")
    })(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    map(
        many_till(
            preceded(multispace0, parse_number),
            preceded(multispace0, eof),
        ),
        |(numbers, _)| numbers,
    )(input)
}
fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    let res = parse_input(&input).expect("failed to parse");
    println!("part one {:?}", solve_part_one(&res.1));
    println!("part two {:?}", solve_part_two(&res.1));
}

#[derive(Debug, Copy, Clone)]
enum NextThing {
    Block,
    Free,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Thing {
    Block(i64),
    Free,
}

fn convert_input(input: &[i64]) -> Vec<Thing> {
    let mut next_thing = NextThing::Block;
    let mut next_id = 0;
    let mut result = Vec::new();

    for size in input {
        match next_thing {
            NextThing::Block => {
                result.extend(repeat(Thing::Block(next_id)).take(*size as usize));
                next_id += 1;
                next_thing = NextThing::Free;
            }
            NextThing::Free => {
                result.extend(repeat(Thing::Free).take(*size as usize));
                next_thing = NextThing::Block;
            }
        }
    }

    result
}

fn rearrange(input: &Vec<Thing>) -> Vec<Thing> {
    let mut start = 0;
    let mut end = input.len() - 1;
    let mut result = input.clone();

    while start < end {
        match (result[start], result[end]) {
            (Thing::Block(_), Thing::Free) => {
                start += 1;
                end -= 1;
            }
            (Thing::Block(_), Thing::Block(_)) => {
                start += 1;
            }
            (Thing::Free, Thing::Block(_)) => {
                result.swap(start, end);
                start += 1;
                end -= 1;
            }
            (Thing::Free, Thing::Free) => {
                end -= 1;
            }
        }
    }
    result
}

fn find_zero_block(vec: &[Thing], target_length: usize) -> Option<Vec<usize>> {
    let mut current_length = 0;
    let mut start_index = 0;

    for (i, &thing) in vec.iter().enumerate() {
        if thing == Thing::Free {
            if current_length == 0 {
                start_index = i;
            }
            current_length += 1;

            if current_length == target_length {
                return Some((start_index..=i).collect());
            }
        } else {
            current_length = 0;
        }
    }

    None
}

fn rearrange_part_two(input: &Vec<Thing>) -> Vec<Thing> {
    let &Thing::Block(id) = input.iter().rev().find(|x| **x != Thing::Free).unwrap() else {
        panic!()
    };

    let mut active_id = id;

    let mut result = input.clone();

    while active_id >= 0 {
        // Get the indices that contain the active_id
        // get indices of the first large enough block of free space
        // swap em all.

        let indices: Vec<usize> = result
            .iter()
            .enumerate()
            .filter(|(_, x)| match x {
                Thing::Block(id) => *id == active_id,
                Thing::Free => false,
            })
            .map(|(idx, _)| idx)
            .collect();

        if let Some(free_space) = find_zero_block(&result, indices.len()) {
            if free_space[0] < indices[0] {
                // swap the i<dices and free_space indices
                indices.iter().zip(free_space).for_each(|(a, b)| {
                    result.swap(*a, b);
                });
            }
        }

        active_id -= 1;
    }
    result
}

fn get_checksum(input: &Vec<Thing>) -> i64 {
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (ix, thing)| match thing {
            Thing::Block(id) => acc + (ix as i64 * id),
            Thing::Free => acc,
        })
}

fn solve_part_one(input: &[i64]) -> i64 {
    get_checksum(&rearrange(&convert_input(input)))
}
fn solve_part_two(input: &[i64]) -> i64 {
    get_checksum(&rearrange_part_two(&convert_input(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_INPUT: &str = "2333133121414131402";
    #[test]
    fn parse_full_input() {
        let result = parse_input(FULL_INPUT);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_convert_input() {
        let result = convert_input(&[1, 2, 3, 4, 5]);
        assert_eq!(
            result,
            vec![
                Thing::Block(0),
                Thing::Free,
                Thing::Free,
                Thing::Block(1),
                Thing::Block(1),
                Thing::Block(1),
                Thing::Free,
                Thing::Free,
                Thing::Free,
                Thing::Free,
                Thing::Block(2),
                Thing::Block(2),
                Thing::Block(2),
                Thing::Block(2),
                Thing::Block(2),
            ]
        );
    }

    #[test]
    fn test_rearrange() {
        let result = rearrange(&vec![
            Thing::Block(0),
            Thing::Free,
            Thing::Free,
            Thing::Block(1),
            Thing::Block(1),
            Thing::Block(1),
            Thing::Free,
            Thing::Free,
            Thing::Free,
            Thing::Free,
            Thing::Block(2),
            Thing::Block(2),
            Thing::Block(2),
            Thing::Block(2),
            Thing::Block(2),
        ]);
        assert_eq!(
            result,
            vec![
                Thing::Block(0),
                Thing::Block(2),
                Thing::Block(2),
                Thing::Block(1),
                Thing::Block(1),
                Thing::Block(1),
                Thing::Block(2),
                Thing::Block(2),
                Thing::Block(2),
                Thing::Free,
                Thing::Free,
                Thing::Free,
                Thing::Free,
                Thing::Free,
                Thing::Free,
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = parse_input("2333133121414131402").expect("failed to parse");
        let result = solve_part_one(&input.1);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input("2333133121414131402").expect("failed to parse");
        let result = solve_part_two(&input.1);
        assert_eq!(result, 2858);
    }
}
