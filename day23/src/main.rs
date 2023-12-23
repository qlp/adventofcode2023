use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

use crate::Direction::{Down, Left, Right, Up};
use crate::Part::{One, Two};
use crate::Tile::{Forrest, Path, Slope};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "94");
    print_answer("one", &one(INPUT), "2010");
    print_answer("two (example)", &two(EXAMPLE), "154");
    print_answer("two", &two(INPUT), "6318");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    TrailMap::from_str(input)
        .expect("a map")
        .longest_trail_length(One)
        .to_string()
}

fn two(input: &str) -> String {
    TrailMap::from_str(input)
        .expect("a map")
        .longest_trail_length(Two)
        .to_string()
}

enum Part {
    One,
    Two,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Trail {
    from: Point,
    to: Point,
    size: usize,
}

impl Display for Trail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -> {} ({})", self.from, self.to, self.size))
    }
}

#[derive(Clone)]
struct TrailMap {
    size: Size,
    tiles: Vec<Tile>,
}

impl TrailMap {
    fn get_tile(&self, point: &Point) -> Tile {
        self.tiles[self.point_to_index(point)]
    }

    fn point_to_index(&self, point: &Point) -> usize {
        point.y * self.size.width + point.x
    }

    fn index_to_point(&self, index: usize) -> Point {
        Point {
            x: index % self.size.width,
            y: index / self.size.width,
        }
    }

    fn start(&self) -> Point {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(index, tile)| match *tile == Path {
                true => Some(self.index_to_point(index)),
                false => None,
            })
            .expect("a starting point")
    }

    fn end(&self) -> Point {
        Point {
            x: self.size.width - 2,
            y: self.size.height - 1,
        }
    }

    fn points_from(&self, point: &Point, from: Option<&Direction>) -> Vec<(Direction, Point)> {
        Direction::all()
            .into_iter()
            .filter(|candidate| match from {
                None => true,
                Some(from) => match self.get_tile(point) {
                    Path => candidate.opposite() != *from,
                    Forrest => panic!("didn't expect to be in the forrest"),
                    Slope(direction) => candidate == &direction,
                },
            })
            .flat_map(|direction| match direction {
                Up => match point.y == 0 {
                    true => None,
                    false => Some((
                        direction,
                        Point {
                            x: point.x,
                            y: point.y - 1,
                        },
                    )),
                },
                Down => match point.y + 1 == self.size.height {
                    true => None,
                    false => Some((
                        direction,
                        Point {
                            x: point.x,
                            y: point.y + 1,
                        },
                    )),
                },
                Left => match point.x == 0 {
                    true => None,
                    false => Some((
                        direction,
                        Point {
                            x: point.x - 1,
                            y: point.y,
                        },
                    )),
                },
                Right => match point.x + 1 == self.size.width {
                    true => None,
                    false => Some((
                        direction,
                        Point {
                            x: point.x + 1,
                            y: point.y,
                        },
                    )),
                },
            })
            .filter(|(to, point)| match self.get_tile(point) {
                Forrest => false,
                Slope(direction) => direction != to.opposite(),
                Path => true,
            })
            .collect()
    }

    fn longest_trail_length(&self, part: Part) -> usize {
        let trails = self.trails_from_start();

        let mut options: Vec<Vec<&Trail>> = vec![vec![trails
            .iter()
            .filter(|trail| trail.from == self.start())
            .collect::<Vec<&Trail>>()
            .single()]];

        let mut completed_trails = vec![];

        while !options.is_empty() {
            // println!("=================================");
            // options.iter().for_each(|option| {
            //     println!("----------------------------------");
            //     println!();
            //     option.iter().for_each(|trail| {
            //         println!("{trail}");
            //     })
            // });

            options = options
                .iter()
                .flat_map(
                    |option| match option.last().expect("at least one").to == self.end() {
                        true => {
                            completed_trails.push(option.clone());
                            vec![]
                        } // found and end path
                        false => trails
                            .iter()
                            .filter(|extension| !option.contains(extension))
                            .filter(|extension| {
                                match part {
                                    One => extension.from == option.last().expect("trails").to,
                                    Two => {
                                        let current_point = match option.len() == 1 {
                                            true => option.single().to, // first track
                                            false => {
                                                let before = option[option.len() - 2];
                                                let last = option.last().expect("last");

                                                match before.to == last.from
                                                    || before.from == last.from
                                                {
                                                    true => last.to,
                                                    false => last.from,
                                                }
                                            }
                                        };

                                        let next_point = match (
                                            current_point == extension.to,
                                            current_point == extension.from,
                                        ) {
                                            (true, false) => Some(extension.from),
                                            (false, true) => Some(extension.to),
                                            (false, false) => None,
                                            (true, true) => panic!("didn't expect a loop"),
                                        };

                                        match next_point {
                                            None => false,
                                            Some(next_point) => !option.iter().any(|track| {
                                                track.from == next_point || track.to == next_point
                                            }),
                                        }
                                    }
                                }
                            })
                            .map(|extension| {
                                let mut new_option = option.clone();
                                new_option.push(extension);

                                new_option
                            })
                            .collect(),
                    },
                )
                .collect()
        }

        completed_trails
            .iter()
            .map(|option| option.iter().fold(0, |acc, trail| acc + trail.size))
            .max()
            .expect("at least one")
    }

    fn trails_from_start(&self) -> Vec<Trail> {
        let mut trails: HashSet<Trail> = HashSet::new();
        let mut added_trails = self.find_trails_from_point(&self.start());

        while !added_trails.is_empty() {
            trails.extend(added_trails.clone());

            added_trails = added_trails
                .iter()
                .filter(|trail| trail.to != self.end())
                .flat_map(|trail_to_start| self.find_trails_from_point(&trail_to_start.to))
                .collect()
        }

        let mut result = Vec::from_iter(trails);
        result.sort_by_key(|trail| self.point_to_index(&trail.from));
        result
    }

    fn find_trails_from_point(&self, point: &Point) -> Vec<Trail> {
        self.points_from(point, None)
            .iter()
            .map(|(direction, start)| self.find_trail_in_direction(point, start, direction))
            .collect()
    }

    fn find_trail_in_direction(&self, origin: &Point, start: &Point, to: &Direction) -> Trail {
        let mut size = 1;

        let mut current_point = *start;
        let mut current_to = *to;

        loop {
            match (size == 1, self.get_tile(&current_point.clone())) {
                (_, Path) | (true, Slope(_)) => match self.end() == current_point {
                    true => {
                        return Trail {
                            from: *origin,
                            to: current_point,
                            size,
                        }
                    } // end of map
                    false => {
                        let points_from = self.points_from(&current_point, Some(&current_to));
                        let (next_to, next_point) = points_from.single();

                        current_to = *next_to;
                        current_point = *next_point;
                        size += 1;
                    }
                },
                (_, Forrest) => panic!("didn't expect to be in forrest"),
                (false, Slope(_)) => {
                    let points_from = self.points_from(&current_point, Some(&current_to));
                    let (_, end_of_trail) = points_from.single();

                    return Trail {
                        from: *origin,
                        to: *end_of_trail,
                        size: size + 1,
                    };
                }
            }
        }
    }
}

impl FromStr for TrailMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = Size::from_str(s)?;

        let tiles = s
            .lines()
            .flat_map(|line| {
                line.chars()
                    .flat_map(|char| Tile::from_str(char.to_string().as_str()))
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Tile>>();

        Ok(Self { size, tiles })
    }
}

impl Display for TrailMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("  ")?;
        (0..self.size.width).try_for_each(|index| f.write_fmt(format_args!("{}", index % 10)))?;
        f.write_char('\n')?;

        self.tiles.iter().enumerate().try_for_each(|(index, tile)| {
            if index % self.size.width == 0 {
                f.write_fmt(format_args!("{} ", (index / self.size.width) % 10))?;
            }

            let result = tile.fmt(f);

            if (index % self.size.width) + 1 == self.size.width {
                f.write_char('\n')?;
            }

            result
        })
    }
}

#[derive(Copy, Clone)]
struct Size {
    width: usize,
    height: usize,
}

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.lines().next() {
            None => Err("expected at least one line".to_string()),
            Some(line) => Ok(Size {
                width: line.len(),
                height: s.lines().count(),
            }),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Path,
    Forrest,
    Slope(Direction),
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Forrest),
            "." => Ok(Path),
            _ => Ok(Slope(Direction::from_str(s)?)),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Forrest => f.write_char('#'),
            Path => f.write_char('.'),
            Slope(direction) => direction.fmt(f),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn all() -> Vec<Direction> {
        vec![Up, Down, Left, Right]
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Up),
            "v" => Ok(Down),
            ">" => Ok(Right),
            "<" => Ok(Left),
            _ => Err(format_args!("Unexpected Slope {s}").to_string()),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
        })
    }
}

trait Single<T> {
    fn single(&self) -> &T;
}

impl<T> Single<T> for Vec<T> {
    fn single(&self) -> &T {
        match self.len() {
            0 => panic!("expected one element, found none"),
            1 => self.first().expect("there is one element"),
            _ => panic!("expected one element, found many"),
        }
    }
}
