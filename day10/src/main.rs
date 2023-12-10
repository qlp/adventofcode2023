use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};

use crate::Connection::*;
use crate::Direction::*;
use crate::Square::{Ground, Pipe, Start};
use crate::State::*;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_1: &str = include_str!("example_1.txt");
const EXAMPLE_2: &str = include_str!("example_2.txt");
const EXAMPLE_3A: &str = include_str!("example_3a.txt");
const EXAMPLE_3B: &str = include_str!("example_3b.txt");
const EXAMPLE_4: &str = include_str!("example_4.txt");
const EXAMPLE_5: &str = include_str!("example_5.txt");

fn main() {
    print_answer("one (example 1)", &one(EXAMPLE_1), "4");
    print_answer("one (example 2)", &one(EXAMPLE_2), "8");
    print_answer("one", &one(INPUT), "6860");
    print_answer("two (example 3a)", &two(EXAMPLE_3A), "4");
    print_answer("two (example 3b)", &two(EXAMPLE_3B), "4");
    print_answer("two (example 4)", &two(EXAMPLE_4), "8");
    print_answer("two (example 5)", &two(EXAMPLE_5), "10");
    print_answer("two", &two(INPUT), "343");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    (World::parse(input).path().len() / 2).to_string()
}

fn two(input: &str) -> String {
    let world = World::parse(input);
    // println!("{world}");

    let mut map = map(&world.path());
    // print_map(&mut map);

    let start = Coordinate { x: 0, y: 0 };
    fill(&mut map, &start, 'O');
    // print_map(&mut map);

    let height = map.len();
    let width = map[0].len();

    let mut space: Option<Coordinate> = Option::None;

    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            if (map[y as usize][x as usize] == ' ' && space.is_none()) {
                space = Some(Coordinate {
                    x: x as i32,
                    y: y as i32,
                });
            }
        })
    });

    fill(&mut map, &space.expect("a space"), '#');
    // print_map(&mut map);

    let mut answer_map: Vec<Vec<char>> = Vec::new();

    (1..height).step_by(2).for_each(|y| {
        let mut answer_line: Vec<char> = Vec::new();

        (1..width).step_by(2).for_each(|x| {
            answer_line.push(map[y][x]);
        });

        answer_map.push(answer_line);
    });

    // print_map(&mut answer_map);

    let answer: i32 = answer_map
        .into_iter()
        .map(|line| line.into_iter().filter(|c| *c == '#').count() as i32)
        .sum();

    answer.to_string()
}

fn fill(mut map: &mut Vec<Vec<char>>, start: &Coordinate, with: char) {
    let mut coordinates_to_fill = vec![start.clone()];
    let replace = map[start.y as usize][start.x as usize];

    while !coordinates_to_fill.is_empty() {
        coordinates_to_fill = coordinates_to_fill
            .iter()
            .flat_map(|c| match map[c.y as usize][c.x as usize] == replace {
                true => {
                    map[c.y as usize][c.x as usize] = with;
                    vec![
                        Coordinate {
                            x: (c.x - 1).max(0),
                            y: c.y,
                        },
                        Coordinate {
                            x: (c.x + 1).min(map[0].len() as i32 - 1),
                            y: c.y,
                        },
                        Coordinate {
                            x: c.x,
                            y: (c.y - 1).max(0),
                        },
                        Coordinate {
                            x: c.x,
                            y: (c.y + 1).min(map.len() as i32 - 1),
                        },
                    ]
                }
                false => vec![],
            })
            .collect()
    }
}

fn print_map(map: &mut Vec<Vec<char>>) {
    println!("----------");
    map.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{}", c));
        println!()
    });
    println!("----------");
}

fn map(path: &Vec<Coordinate>) -> Vec<Vec<char>> {
    let width = path.iter().max_by_key(|c| c.x).expect("y").x as usize;
    let heigth = path.iter().max_by_key(|c| c.y).expect("x").y as usize;

    let multiplier = 2usize;
    let border = 1usize;

    let map_width = width * multiplier + 2 * border * multiplier;
    let map_height = heigth * multiplier + 2 * border * multiplier;

    let mut result: Vec<Vec<char>> = Vec::new();

    (0..map_height).for_each(|_| result.push(vec![' '; map_width]));

    let mut circular_path = Vec::from_iter(path.into_iter());
    circular_path.push(&path[0]);

    circular_path.windows(2).for_each(|c| {
        let from = &c[0];
        let to = &c[1];

        let from_x = from.x.min(to.x) as usize * multiplier + border;
        let from_y = from.y.min(to.y) as usize * multiplier + border;

        let to_x = from.x.max(to.x) as usize * multiplier + border;
        let to_y = from.y.max(to.y) as usize * multiplier + border;

        (from_x..=to_x).into_iter().for_each(|x| {
            (from_y..=to_y).into_iter().for_each(|y| {
                result[y][x] = '*';
            })
        });
    });

    result
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum State {
    Inside(i32),
    Outside,
}

#[derive(Clone)]
struct World {
    squares: Vec<Vec<Square>>,
}

impl World {
    fn start(&self) -> (Coordinate, Direction) {
        let coordinate = self
            .squares
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find_map(|(x, square)| match square {
                        Start => Some(x),
                        _ => None,
                    })
                    .map(|x| Coordinate {
                        x: x as i32,
                        y: y as i32,
                    })
            })
            .expect("a starting point");

        let direction = [North, South, East, West]
            .iter()
            .find(|d| {
                let square = self.square_at(&d.next(&coordinate));
                match square {
                    Pipe(p) => p.directions().contains(&d.opposite()),
                    Ground => false,
                    Start => false,
                }
            })
            .expect("at least one");

        (coordinate, direction.clone())
    }

    fn square_at(&self, coordinate: &Coordinate) -> &Square {
        match (coordinate.x, coordinate.y) {
            (..=-1, _) => &Ground,
            (_, ..=-1) => &Ground,
            (x, y) => &self.squares[y as usize][x as usize],
        }
    }

    fn parse(input: &str) -> Self {
        Self {
            squares: input
                .lines()
                .map(|l| l.chars().map(|c| Square::parse(c)).collect())
                .collect(),
        }
    }

    fn only_path(&self) -> Self {
        let path = self.path();
        let size = self.size();

        Self {
            squares: (0..size.height as i32)
                .map(|y| {
                    (0..size.width as i32)
                        .map(|x| match path.iter().find(|c| **c == Coordinate { x, y }) {
                            None => Ground,
                            Some(c) => Start,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn path(&self) -> Vec<Coordinate> {
        let (start, direction) = self.start();

        let mut current_coordinate: Coordinate = start;
        let mut current_direction: Direction = direction.clone();
        let mut result: Vec<Coordinate> = Vec::new();

        loop {
            result.push(current_coordinate.clone());
            current_coordinate = current_direction.next(&current_coordinate);
            let next_square = self.square_at(&current_coordinate);

            current_direction = match next_square {
                Pipe(c) => c.next_direction(&current_direction).clone(),
                Ground => panic!("expecting pipe or start"),
                Start => break,
            }
        }

        result
    }

    fn size(&self) -> Size {
        Size {
            width: self.squares.first().expect("one").len(),
            height: self.squares.len(),
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.squares.iter().enumerate().for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .for_each(|(x, s)| s.fmt(f).expect("ok"));
            f.write_char('\n').expect("ok")
        });

        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Square {
    Pipe(Connection),
    Ground,
    Start,
}

impl Square {
    fn parse(input: char) -> Self {
        match input {
            '|' => Pipe(NS),
            '-' => Pipe(EW),
            'L' => Pipe(NE),
            'J' => Pipe(NW),
            '7' => Pipe(SW),
            'F' => Pipe(SE),
            '.' => Ground,
            'S' => Start,
            _ => {
                dbg!(input);
                panic!("unexpected character")
            }
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe(p) => p.fmt(f),
            Ground => f.write_char(' '),
            Start => f.write_char('S'),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    South,
    North,
    East,
    West,
}

impl Direction {
    fn next(&self, from: &Coordinate) -> Coordinate {
        match self {
            South => Coordinate {
                x: from.x,
                y: from.y + 1,
            },
            North => Coordinate {
                x: from.x,
                y: from.y - 1,
            },
            East => Coordinate {
                x: from.x + 1,
                y: from.y,
            },
            West => Coordinate {
                x: from.x - 1,
                y: from.y,
            },
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            South => North,
            North => South,
            East => West,
            West => East,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Connection {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Connection {
    fn directions(&self) -> [Direction; 2] {
        match self {
            NS => [North, South],
            EW => [East, West],
            NE => [North, East],
            NW => [North, West],
            SW => [South, West],
            SE => [South, East],
        }
    }
}

impl Connection {
    fn next_direction(&self, from: &Direction) -> Direction {
        let directions = self.directions();

        let one = directions[0].clone();
        let two = directions[1].clone();

        match one == from.opposite() {
            true => two,
            false => one,
        }
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            NS => '│',
            EW => '─',
            NE => '└',
            NW => '┘',
            SW => '┐',
            SE => '┌',
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct Size {
    width: usize,
    height: usize,
}
