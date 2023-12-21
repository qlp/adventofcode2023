use bit_set::BitSet;
use std::fmt::{Display, Formatter, Write};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE, 6), "16");
    print_answer("one", &one(INPUT, 64), "");
    // print_answer("two (example)", &two(EXAMPLE), "");
    // print_answer("two", &two(INPUT), "");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str, number_of_steps: usize) -> String {
    let world = World::parse(input);

    println!("{world}");

    world.reached(number_of_steps).to_string()
}

fn two(input: &str) -> String {
    String::new()
}

struct World {
    size: Size,
    start: Point,
    map: BitSet,
}

impl World {
    fn parse(input: &str) -> Self {
        let size = Size {
            width: input.lines().next().expect("at least one line").len(),
            height: input.lines().count(),
        };
        let start = input
            .find('S')
            .map(|start| Point {
                x: start % (size.width + 1), // +1 to compensate for new-line chars
                y: start / (size.width + 1),
            })
            .expect("a start point");

        let map = BitSet::from_iter(
            input
                .lines()
                .enumerate()
                .flat_map(|(row_index, line)| {
                    line.chars()
                        .enumerate()
                        .flat_map(move |(column_index, char)| match char {
                            '.' | 'S' => None,
                            '#' => Some(row_index * size.width + column_index),
                            _ => panic!("unexpected char"),
                        })
                })
                .collect::<Vec<usize>>(),
        );

        Self { size, start, map }
    }

    fn reached(&self, number_of_steps: usize) -> usize {
        let mut reached = BitSet::with_capacity(self.size.surface());

        reached.insert(self.index(&self.start));

        (1..=number_of_steps).for_each(|step| {
            reached.extend(
                reached
                    .iter()
                    .flat_map(|point_index| {
                        [
                            Direction::Left,
                            Direction::Right,
                            Direction::Up,
                            Direction::Down,
                        ]
                        .into_iter()
                        .filter_map(|direction| self.walk(&self.point(point_index), &direction))
                        .map(|point| self.index(&point))
                        .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<usize>>(),
            );
        });

        reached
            .iter()
            .filter(|reached_index| {
                let reached_point = self.point(*reached_index);

                match reached_point.y % 2 == self.start.y % 2 {
                    true => reached_point.x % 2 == self.start.x % 2,
                    false => reached_point.x % 2 != self.start.x % 2,
                }
            })
            .count()
    }

    fn index(&self, point: &Point) -> usize {
        point.y * self.size.width + point.x
    }

    fn point(&self, index: usize) -> Point {
        Point {
            x: index % self.size.width,
            y: index / self.size.width,
        }
    }

    fn walk(&self, point: &Point, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::Up => match point.y == 0 {
                true => None,
                false => Some(Point {
                    x: point.x,
                    y: point.y - 1,
                }),
            },
            Direction::Down => match point.y == self.size.height - 1 {
                true => None,
                false => Some(Point {
                    x: point.x,
                    y: point.y + 1,
                }),
            },
            Direction::Left => match point.x == 0 {
                true => None,
                false => Some(Point {
                    x: point.x - 1,
                    y: point.y,
                }),
            },
            Direction::Right => match point.x == self.size.width - 1 {
                true => None,
                false => Some(Point {
                    x: point.x + 1,
                    y: point.y,
                }),
            },
        }
        .and_then(|point| match self.map.contains(self.index(&point)) {
            true => None,
            false => Some(point),
        })
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.size.height).for_each(|line_index| {
            (0..self.size.width).for_each(|column_index| {
                f.write_char(
                    match self
                        .map
                        .contains(line_index * self.size.width + column_index)
                    {
                        true => '#',
                        false => match line_index == self.start.y && column_index == self.start.x {
                            true => 'S',
                            false => '.',
                        },
                    },
                )
                .unwrap()
            });
            f.write_char('\n').unwrap();
        });

        Ok(())
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

struct Size {
    width: usize,
    height: usize,
}

impl Size {
    fn surface(&self) -> usize {
        self.width * self.height
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("w:{}, h:{})", self.width, self.height))
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
