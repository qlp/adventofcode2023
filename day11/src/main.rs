use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "374");
    print_answer("one", &one(INPUT), "9556712");
    print_answer("two (example)", &two(EXAMPLE, 10), "1030");
    print_answer("two (example)", &two(EXAMPLE, 100), "8410");
    print_answer("two (example)", &two(INPUT, 1_000_000), "678626199476");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    total_distance(input, 2).to_string()
}

fn two(input: &str, expansion: u64) -> String {
    total_distance(input, expansion).to_string()
}

fn total_distance(input: &str, expansion: u64) -> u64 {
    World::parse(input)
        .expand(expansion)
        .connections()
        .iter()
        .map(|(a, b)| a.distance(b))
        .sum::<u64>()
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
                                x: x as u64,
                                y: y as u64,
                            }),
                            _ => None,
                        })
                        .collect::<Vec<Coordinate>>()
                })
                .collect(),
        }
    }

    fn transpose(&self) -> Self {
        Self {
            planets: self
                .planets
                .iter()
                .map(|c| Coordinate { x: c.y, y: c.x })
                .collect(),
        }
    }

    fn expand(&self, expansion: u64) -> Self {
        self.expand_rows(expansion)
            .transpose()
            .expand_rows(expansion)
            .transpose()
    }

    fn expand_rows(&self, expansion: u64) -> Self {
        let height = self.planets.iter().fold(0, |max, c| max.max(c.y)) + 1;

        let empty_rows: Vec<u64> = (0..height)
            .filter(|y| !self.planets.iter().any(|c| c.y == *y))
            .collect();

        Self {
            planets: self
                .planets
                .iter()
                .map(|p| {
                    let y = p.y
                        + (expansion - 1) * empty_rows.iter().filter(|y| *y < &p.y).count() as u64;

                    Coordinate { x: p.x, y }
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
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: u64,
    y: u64,
}

impl Coordinate {
    fn distance(&self, to: &Coordinate) -> u64 {
        self.x.abs_diff(to.x) + self.y.abs_diff(to.y)
    }
}
