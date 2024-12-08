use std::{
    collections::{HashMap, HashSet},
    fs,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, none_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    let res = parse_input(&input).expect("failed to parse");
    println!("part one {:?}", solve_part_one(&res.1));
    println!("part two {:?}", solve_part_two(&res.1));
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Characters {
    Empty,
    Char(char),
}
fn parse_line(input: &str) -> IResult<&str, Vec<Characters>> {
    many1(alt((
        map(tag("."), |_| Characters::Empty),
        map(none_of("\n\r"), |x| Characters::Char(x)),
    )))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Characters>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn generate_position_pairs(positions: &[(usize, usize)]) -> Vec<((usize, usize), (usize, usize))> {
    let mut pairs = Vec::new();
    for i in 0..positions.len() {
        for j in 0..positions.len() {
            if i != j {
                pairs.push((positions[i], positions[j]));
            }
        }
    }
    pairs
}

fn get_lots_of_antinodes_from_pair(
    maxy: usize,
    maxx: usize,
    pair: ((usize, usize), (usize, usize)),
) -> Vec<(i32, i32)> {
    let vector = (
        pair.1 .0 as i32 - pair.0 .0 as i32,
        pair.1 .1 as i32 - pair.0 .1 as i32,
    );

    let mut nodes = vec![
        (pair.0 .0 as i32, pair.0 .1 as i32),
        (pair.1 .0 as i32, pair.1 .1 as i32),
    ];

    let mut distance = 1;
    loop {
        let next = (
            pair.0 .0 as i32 + (vector.0 * distance),
            pair.0 .1 as i32 + (vector.1 * distance),
        );

        if next.0 > maxy as i32 || next.1 > maxx as i32 || next.0 < 0 || next.1 < 0 {
            break;
        } else {
            nodes.push(next);
            distance += 1;
        }
    }
    distance = 1;
    loop {
        let next = (
            pair.1 .0 as i32 - (vector.0 * distance),
            pair.1 .1 as i32 - (vector.1 * distance),
        );

        if next.0 > maxy as i32 || next.1 > maxx as i32 || next.0 < 0 || next.1 < 0 {
            break;
        } else {
            nodes.push(next);
            distance += 1;
        }
    }
    nodes
}

fn get_antinodes_from_pair(
    maxy: usize,
    maxx: usize,
    pair: ((usize, usize), (usize, usize)),
) -> Vec<(i32, i32)> {
    let vector = (
        pair.1 .0 as i32 - pair.0 .0 as i32,
        pair.1 .1 as i32 - pair.0 .1 as i32,
    );
    vec![
        (pair.0 .0 as i32 - vector.0, pair.0 .1 as i32 - vector.1),
        (pair.1 .0 as i32 + vector.0, pair.1 .1 as i32 + vector.1),
    ]
    .iter()
    .filter(|(x, y)| *x >= 0 && *x < maxx as i32 && *y >= 0 && *y < maxy as i32)
    .copied() // or .cloned() if you prefer
    .collect()
}

fn solve_part_one(input: &Vec<Vec<Characters>>) -> i32 {
    let mut map_of_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (ypos, line) in input.iter().enumerate() {
        for (xpos, character) in line.iter().enumerate() {
            match character {
                Characters::Char(c) => {
                    map_of_positions
                        .entry(*c)
                        .or_insert(Vec::new())
                        .push((xpos, ypos));
                }
                _ => {}
            }
        }
    }
    // Generate all pairs for each character
    let mut all_pairs: HashSet<Vec<((usize, usize), (usize, usize))>> = HashSet::new();
    for (_, positions) in &map_of_positions {
        all_pairs.insert(generate_position_pairs(positions));
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for pairs in all_pairs {
        for pair in pairs {
            let antinodes_from_pair = get_antinodes_from_pair(input[0].len(), input.len(), pair);
            antinodes_from_pair.iter().for_each(|x| {
                antinodes.insert(*x);
            });
        }
    }
    return antinodes.len().try_into().unwrap();
}

fn solve_part_two(input: &Vec<Vec<Characters>>) -> i32 {
    let mut map_of_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (ypos, line) in input.iter().enumerate() {
        for (xpos, character) in line.iter().enumerate() {
            match character {
                Characters::Char(c) => {
                    map_of_positions
                        .entry(*c)
                        .or_insert(Vec::new())
                        .push((xpos, ypos));
                }
                _ => {}
            }
        }
    }
    // Generate all pairs for each character
    let mut all_pairs: HashSet<Vec<((usize, usize), (usize, usize))>> = HashSet::new();
    for (_, positions) in &map_of_positions {
        all_pairs.insert(generate_position_pairs(positions));
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for pairs in all_pairs {
        for pair in pairs {
            let antinodes_from_pair =
                get_lots_of_antinodes_from_pair(input.len() - 1, input[0].len() - 1, pair);
            antinodes_from_pair.iter().for_each(|x| {
                antinodes.insert(*x);
            });
        }
    }

    return antinodes.len().try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_INPUT: &str = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";

    #[test]
    fn parse_line_test() {
        let input = "......A.....";
        let result = parse_line(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            vec![
                Characters::Empty,
                Characters::Empty,
                Characters::Empty,
                Characters::Empty,
                Characters::Empty,
                Characters::Empty,
                Characters::Char('A'),
                Characters::Empty,
                Characters::Empty,
                Characters::Empty,
                Characters::Empty,
                Characters::Empty,
            ]
        );
    }
    #[test]
    fn parse_full_input() {
        let result = parse_input(FULL_INPUT);
        assert!(result.is_ok());
    }

    #[test]
    fn test_solution_one() {
        let result = parse_input(FULL_INPUT).expect("failed to parse");
        let solution = solve_part_one(&result.1);
        assert_eq!(solution, 14)
    }

    #[test]
    fn test_solution_two() {
        let result = parse_input(FULL_INPUT).expect("failed to parse");
        let solution = solve_part_two(&result.1);
        assert_eq!(solution, 34)
    }
}
