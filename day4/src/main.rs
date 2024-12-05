use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Characters {
    X,
    M,
    A,
    S,
}

fn xmas_char(input: &str) -> IResult<&str, Characters> {
    map(
        alt((char('X'), char('M'), char('A'), char('S'))),
        |c| match c {
            'X' => Characters::X,
            'M' => Characters::M,
            'A' => Characters::A,
            'S' => Characters::S,
            _ => unreachable!(),
        },
    )(input)
}

fn parse_chars(input: &str) -> IResult<&str, Vec<Characters>> {
    many1(xmas_char)(input)
}
fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Characters>>> {
    separated_list1(line_ending, parse_chars)(input)
}

fn add_vector((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    (x1 + x2, y1 + y2)
}

fn get_xmas(
    input: &[Vec<Characters>],
    starting_cordinates: (i32, i32),
    vector: (i32, i32),
) -> bool {
    let coordinates = [
        starting_cordinates,
        add_vector(starting_cordinates, vector),
        add_vector(starting_cordinates, add_vector(vector, vector)),
        add_vector(
            starting_cordinates,
            add_vector(vector, add_vector(vector, vector)),
        ),
    ];

    let res: Vec<Characters> = coordinates
        .iter()
        .filter_map(|(y, x)| {
            // First check if y is in bounds
            input
                .get(*y as usize)
                .and_then(|row| {
                    // Then check if x is in bounds for that row
                    row.get(*x as usize)
                })
                .cloned() // Clone the value since we got it by reference
        })
        .collect();

    res == [Characters::X, Characters::M, Characters::A, Characters::S]
}

fn solve_part_one(input: &[Vec<Characters>]) -> i32 {
    let vectors = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    let mut count = 0;
    for (ypos, row) in input.iter().enumerate() {
        for (xpos, c) in row.iter().enumerate() {
            for vector in vectors.iter() {
                if c == &Characters::X && get_xmas(input, (ypos as i32, xpos as i32), *vector) {
                    count += 1
                }
            }
        }
    }
    count
}

fn get_cross(input: &[Vec<Characters>], starting_cordinates: (i32, i32)) -> bool {
    let coordinates = [
        add_vector(starting_cordinates, (-1, -1)),
        add_vector(starting_cordinates, (-1, 1)),
        add_vector(starting_cordinates, (1, 1)),
        add_vector(starting_cordinates, (1, -1)),
    ];

    let res: Vec<Characters> = coordinates
        .iter()
        .filter_map(|(y, x)| {
            // First check if y is in bounds
            input
                .get(*y as usize)
                .and_then(|row| {
                    // Then check if x is in bounds for that row
                    row.get(*x as usize)
                })
                .cloned() // Clone the value since we got it by reference
        })
        .collect();

    // The characters should be MMSS, but possibly rotated
    let valid_patterns = [
        vec![Characters::M, Characters::M, Characters::S, Characters::S],
        vec![Characters::S, Characters::M, Characters::M, Characters::S],
        vec![Characters::S, Characters::S, Characters::M, Characters::M],
        vec![Characters::M, Characters::S, Characters::S, Characters::M],
    ];

    valid_patterns.contains(&res)
}

fn solve_part_two(input: &[Vec<Characters>]) -> i32 {
    let mut count = 0;
    for (ypos, row) in input.iter().enumerate() {
        for (xpos, c) in row.iter().enumerate() {
            if c == &Characters::A && get_cross(input, (ypos as i32, xpos as i32)) {
                count += 1
            }
        }
    }
    count
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
    fn parse_many_mul() {
        let input = "MMMSXXMASM";
        let result = parse_chars(input);
        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().1.len(), 10);
        assert_eq!(
            result.unwrap().1,
            vec![
                Characters::M,
                Characters::M,
                Characters::M,
                Characters::S,
                Characters::X,
                Characters::X,
                Characters::M,
                Characters::A,
                Characters::S,
                Characters::M
            ]
        );
    }

    #[test]
    fn parse_full_input() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let result = parse_input(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1[0],
            vec![
                Characters::M,
                Characters::M,
                Characters::M,
                Characters::S,
                Characters::X,
                Characters::X,
                Characters::M,
                Characters::A,
                Characters::S,
                Characters::M
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let parsed = parse_input(input);
        let solution = solve_part_one(&parsed.unwrap().1);
        assert_eq!(solution, 18)
    }

    #[test]
    fn test_part_two() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let parsed = parse_input(input);
        let solution = solve_part_two(&parsed.unwrap().1);
        assert_eq!(solution, 9)
    }
}
