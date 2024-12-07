use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::fs;
use std::io::Write;
use std::{collections::HashSet, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapThing {
    Obstacle,
    Empty,
    // Player always starts facing up
    Player(Direction),
}

fn parse_obstacle(input: &str) -> IResult<&str, MapThing> {
    map(tag("#"), |_| MapThing::Obstacle)(input)
}
fn parse_empty(input: &str) -> IResult<&str, MapThing> {
    map(tag("."), |_| MapThing::Empty)(input)
}
fn parse_player(input: &str) -> IResult<&str, MapThing> {
    map(tag("^"), |_| MapThing::Player(Direction::Up))(input)
}
fn parse_map_thing(input: &str) -> IResult<&str, MapThing> {
    alt((parse_obstacle, parse_empty, parse_player))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<MapThing>> {
    many1(parse_map_thing)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<MapThing>>> {
    separated_list1(line_ending, parse_line)(input)
}

struct Player {
    x: usize,
    y: usize,
    direction: Direction,
}
struct GameState {
    player: Player,
    map: Vec<Vec<MapThing>>,
}

impl GameState {
    fn create(input: &[Vec<MapThing>]) -> Self {
        // Create a deep clone of the input
        let mut map: Vec<Vec<MapThing>> = input.iter().map(|row| row.to_vec()).collect();

        let mut player = None;
        'outer: for (ypos, row) in input.iter().enumerate() {
            for (xpos, item) in row.iter().enumerate() {
                if let MapThing::Player(Direction::Up) = item {
                    map[ypos][xpos] = MapThing::Empty;
                    player = Some(Player {
                        x: xpos,
                        y: ypos,
                        direction: Direction::Up,
                    });
                    break 'outer;
                }
            }
        }

        GameState {
            player: player.unwrap(),
            map,
        }
    }

    fn out_of_bounds(&self, x: usize, y: usize) -> bool {
        x >= self.map[0].len() || y >= self.map.len()
    }

    fn tick(mut self) -> Self {
        loop {
            let vec = match self.player.direction {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            let next_position = (self.player.x as i32 + vec.0, self.player.y as i32 + vec.1);

            if let Some(MapThing::Obstacle) = self
                .map
                .get(next_position.1 as usize)
                .and_then(|x| x.get(next_position.0 as usize))
            {
                self.player.direction = match self.player.direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            } else {
                self.player.x = next_position.0 as usize;
                self.player.y = next_position.1 as usize;
                break;
            }
        }
        self
    }
}
fn solve_part_one(input: &[Vec<MapThing>]) -> i32 {
    let mut game_state = GameState::create(input);
    let mut visited = HashSet::new();

    while !game_state.out_of_bounds(game_state.player.x, game_state.player.y) {
        visited.insert((game_state.player.x, game_state.player.y));
        game_state = game_state.tick();
    }
    visited.len() as i32
}

fn solve_part_two(input: &[Vec<MapThing>]) -> i32 {
    let mut test_obstacles = vec![];
    let map: Vec<Vec<MapThing>> = input.iter().map(|row| row.to_vec()).collect();
    for (ypos, row) in map.iter().enumerate() {
        for (xpos, item) in row.iter().enumerate() {
            if item == &MapThing::Empty {
                test_obstacles.push((xpos, ypos));
            }
        }
    }

    // Tied myself in circles trying to get the following to work.
    // Eventually I decided to do the brute horrible approach because it's late and I'm tired

    //         if let MapThing::Empty = item {
    //             let mut distance = 0;
    //             std::io::stdout().flush().unwrap();

    //             // I actaully  need to repeat this process in the four directions :/
    //             'outer: loop {
    //                 // Top right corner (original)
    //                 let top_right = [
    //                     ((xpos as i32) + 1 + (distance as i32), (ypos as i32) + 1),
    //                     (
    //                         (xpos as i32) + (distance as i32),
    //                         (ypos as i32) + 2 + (distance as i32),
    //                     ),
    //                     ((xpos as i32) - 1, (ypos as i32) + 1 + (distance as i32)),
    //                 ];

    //                 // Top left corner
    //                 let top_left = [
    //                     ((xpos as i32) - 1 - (distance as i32), (ypos as i32) + 1),
    //                     (
    //                         (xpos as i32) - (distance as i32),
    //                         (ypos as i32) + 2 + (distance as i32),
    //                     ),
    //                     ((xpos as i32) + 1, (ypos as i32) + 1 + (distance as i32)),
    //                 ];

    //                 // Bottom right corner
    //                 let bottom_right = [
    //                     ((xpos as i32) + 1 + (distance as i32), (ypos as i32) - 1),
    //                     (
    //                         (xpos as i32) + (distance as i32),
    //                         (ypos as i32) - 2 - (distance as i32),
    //                     ),
    //                     ((xpos as i32) - 1, (ypos as i32) - 1 - (distance as i32)),
    //                 ];

    //                 // Bottom left corner
    //                 let bottom_left = [
    //                     ((xpos as i32) - 1 - (distance as i32), (ypos as i32) - 1),
    //                     (
    //                         (xpos as i32) - (distance as i32),
    //                         (ypos as i32) - 2 - (distance as i32),
    //                     ),
    //                     ((xpos as i32) + 1, (ypos as i32) - 1 - (distance as i32)),
    //                 ];
    //                 let positions = [top_right, top_left, bottom_right, bottom_left];
    //                 for corner_positions in positions.iter() {
    //                     let mut found_obstacles = 0;

    //                     for (x, y) in corner_positions.iter() {
    //                         if x < &0 || y < &0 {
    //                             break 'outer;
    //                         }
    //                         match map.get(*y as usize).and_then(|row| row.get(*x as usize)) {
    //                             Some(MapThing::Obstacle) => found_obstacles += 1,
    //                             None => break 'outer,
    //                             _ => {}
    //                         }
    //                     }
    //                     println!("found{:?}", found_obstacles);

    //                     if found_obstacles == 3 {
    //                         test_obstacles.push((xpos, ypos));
    //                         break;
    //                     }
    //                 }

    //                 distance += 1;
    //             }
    //         }
    //     }
    // }

    std::io::stdout().flush().unwrap();

    test_obstacles.iter().fold(0, |acc, (x, y)| {
        if check_for_loop(input, *x, *y) {
            acc + 1
        } else {
            acc
        }
    })
}

fn check_for_loop(input: &[Vec<MapThing>], x: usize, y: usize) -> bool {
    let mut map: Vec<Vec<MapThing>> = input.iter().map(|row| row.to_vec()).collect();
    map[y][x] = MapThing::Obstacle;
    // create a new game board, with the test_obstacle added
    let mut game_state = GameState::create(&map);
    // create a map of player position and facing. If you are ever in the same position and direction twice you are loopin.
    let mut visited = HashSet::new();

    while !game_state.out_of_bounds(game_state.player.x, game_state.player.y) {
        if visited.contains(&(
            game_state.player.x,
            game_state.player.y,
            game_state.player.direction,
        )) {
            return true;
        }

        visited.insert((
            game_state.player.x,
            game_state.player.y,
            game_state.player.direction,
        ));
        game_state = game_state.tick();
    }
    false
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
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let result = parse_input(input);
        assert!(result.is_ok());
        println!("{:?}", result);
        assert!(result.unwrap().1.len() == 10);
    }
    #[test]
    fn part_one() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let parsed = parse_input(input).expect("failed to parse");
        let solution = solve_part_one(&parsed.1);
        assert_eq!(solution, 41)
    }

    #[test]
    fn check_loops() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let parsed = parse_input(input).expect("failed to parse");
        let solution = check_for_loop(&parsed.1, 3, 6);
        assert_eq!(solution, true)
    }

    #[test]
    fn part_two() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let parsed = parse_input(input).expect("failed to parse");
        let solution = solve_part_two(&parsed.1);
        assert_eq!(solution, 6)
    }
}
