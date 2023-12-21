use bit_set::BitSet;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE, 6), "16");
    print_answer("one", &one(INPUT, 64), "3751");
    print_answer("two (example) - 6", &two(EXAMPLE, 6), "16");
    print_answer("two (example) - 10", &two(EXAMPLE, 10), "50");
    print_answer("two (example) - 50", &two(EXAMPLE, 50), "1594");
    print_answer("two (example) - 100", &two(EXAMPLE, 100), "6536");
    print_answer("two (example) - 500", &two(EXAMPLE, 500), "167004");
    print_answer("two (example) - 1000", &two(EXAMPLE, 1000), "668697");
    print_answer("two (example) - 5000", &two(EXAMPLE, 5000), "16733044");
    print_answer("two", &two(INPUT, 26501365), "");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str, number_of_steps: usize) -> String {
    World::parse(input).reached(number_of_steps).to_string()
}

fn two(input: &str, number_of_steps: usize) -> String {
    World::parse(input).reached(number_of_steps).to_string()
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
                x: start as i64 % (size.width as i64 + 1), // +1 to compensate for new-line chars
                y: start as i64 / (size.width as i64 + 1),
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
        let mut reached: HashSet<Point> = HashSet::new();

        reached.insert(self.start);

        (1..=number_of_steps).for_each(|_| {
            reached.extend(
                reached
                    .iter()
                    .flat_map(|point| {
                        [
                            Direction::Left,
                            Direction::Right,
                            Direction::Up,
                            Direction::Down,
                        ]
                        .into_iter()
                        .filter_map(|direction| self.walk(&point, &direction))
                        .collect::<Vec<Point>>()
                    })
                    .collect::<Vec<Point>>(),
            );
        });

        let min_x = reached
            .iter()
            .min_by_key(|point| point.x)
            .expect("points")
            .x;
        let max_x = reached
            .iter()
            .max_by_key(|point| point.x)
            .expect("points")
            .x;
        let min_y = reached
            .iter()
            .min_by_key(|point| point.y)
            .expect("points")
            .y;
        let max_y = reached
            .iter()
            .max_by_key(|point| point.y)
            .expect("points")
            .y;

        (min_y..=max_y).for_each(|y| {
            (min_x..=max_x).for_each(|x| {
                let point = Point { x, y };
                let did_reach = reached.contains(&point);
                let is_blocked = self.blocked(&point);
                let is_start = point == self.start;

                print!(
                    "{}",
                    match (did_reach, is_blocked, is_start) {
                        (true, false, true) => 'S',
                        (true, false, false) => 'O',
                        (false, true, false) => '#',
                        (false, false, false) => '.',
                        _ => panic!("unexpected"),
                    }
                );
            });
            println!();
        });

        reached
            .into_iter()
            .filter(
                |reached_point| match mod_pos(reached_point.y, 2) == mod_pos(self.start.y, 2) {
                    true => mod_pos(reached_point.x, 2) == mod_pos(self.start.x, 2),
                    false => mod_pos(reached_point.x, 2) != mod_pos(self.start.x, 2),
                },
            )
            .count()
    }

    fn blocked(&self, point: &Point) -> bool {
        self.map.contains(self.index_on_map(point))
    }

    fn index_on_map(&self, point: &Point) -> usize {
        let x = mod_pos(point.x, self.size.width as i64) as usize;
        let y = mod_pos(point.y, self.size.width as i64) as usize;

        y * self.size.width + x
    }

    fn walk(&self, point: &Point, direction: &Direction) -> Option<Point> {
        let candidate = match direction {
            Direction::Up => Point {
                x: point.x,
                y: point.y + 1,
            },
            Direction::Down => Point {
                x: point.x,
                y: point.y - 1,
            },
            Direction::Left => Point {
                x: point.x - 1,
                y: point.y,
            },
            Direction::Right => Point {
                x: point.x + 1,
                y: point.y,
            },
        };

        match self.blocked(&candidate) {
            true => None,
            false => Some(candidate),
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.size.height).for_each(|line_index| {
            (0..self.size.width).for_each(|column_index| {
                f.write_char(
                    match self
                        .map
                        .contains((line_index * self.size.width + column_index) as usize)
                    {
                        true => '#',
                        false => match line_index == self.start.y as usize
                            && column_index == self.start.x as usize
                        {
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

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
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

fn mod_pos(value: i64, division: i64) -> i64 {
    ((value % division) + division) % division
}
