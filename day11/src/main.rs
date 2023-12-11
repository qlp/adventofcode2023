use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "374");
    print_answer("one", &one(INPUT), "9556712");
    // print_answer("two (example)", &two(EXAMPLE), "");
    // print_answer("two", &two(INPUT), "");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    World::parse(input)
        .expand()
        .connections()
        .iter()
        .map(|(a, b)| a.distance(b))
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    String::new()
}

#[derive(Debug, Clone)]
struct World {
    planets: HashSet<Coordinate>,
}

impl World {
    fn parse(input: &str) -> Self {
        Self {
            planets: input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(|(x, char)| match char {
                            '#' => Some(Coordinate {
                                x: x as u32,
                                y: y as u32,
                            }),
                            _ => None,
                        })
                        .collect::<Vec<Coordinate>>()
                })
                .collect(),
        }
    }

    fn size(&self) -> Size {
        let (max_x, max_y) = self
            .planets
            .iter()
            .fold((0, 0), |(x, y), c| (x.max(c.x), y.max(c.y)));

        Size {
            width: max_x + 1,
            height: max_y + 1,
        }
    }

    fn expand(&self) -> Self {
        let size = self.size();
        let empty_x: Vec<u32> = (0..size.width)
            .filter(|x| !self.planets.iter().any(|c| c.x == *x))
            .collect();
        let empty_y: Vec<u32> = (0..size.height)
            .filter(|y| !self.planets.iter().any(|c| c.y == *y))
            .collect();

        Self {
            planets: self
                .planets
                .iter()
                .map(|p| {
                    let x = p.x + empty_x.iter().filter(|x| *x < &p.x).count() as u32;
                    let y = p.y + empty_y.iter().filter(|y| *y < &p.y).count() as u32;

                    Coordinate { x, y }
                })
                .collect(),
        }
    }

    fn connections(&self) -> Vec<(&Coordinate, &Coordinate)> {
        let coordinates = Vec::from_iter(self.planets.iter());

        (0..(coordinates.len() - 1))
            .flat_map(|i| {
                ((i + 1)..coordinates.len())
                    .map(|j| (coordinates[i], coordinates[j]))
                    .collect::<Vec<(&Coordinate, &Coordinate)>>()
            })
            .collect()
    }

    fn print(&self) {
        let size = self.size();
        (0..size.height).for_each(|y| {
            (0..size.width).for_each(|x| match self.planets.contains(&Coordinate { x, y }) {
                true => print!("#"),
                false => print!("."),
            });
            println!();
        });
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn distance(&self, to: &Coordinate) -> u32 {
        self.x.abs_diff(to.x) + self.y.abs_diff(to.y)
    }
}

impl PartialOrd<Coordinate> for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => self.x.partial_cmp(&other.x),
            other => other,
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

struct Size {
    width: u32,
    height: u32,
}
