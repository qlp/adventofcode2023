use bit_set::BitSet;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE, 6), "16");
    print_answer("one", &one(INPUT, 64), "3751");
    print_answer("two", &two(INPUT, 26501365), "619407349431167");
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
    let world = World::parse(input);

    let size = world.size.width;

    let steps_to_leave_center = ((size - 1) / 2) as u64;
    let blocks_in_between = (number_of_steps as u64 - steps_to_leave_center) / size as u64;

    let points_on_axis: u64 = [
        Point {
            x: (size as usize - 1) / 2,
            y: size as usize - 1,
        }, // up
        Point {
            x: (size as usize - 1) / 2,
            y: 0,
        }, // down
        Point {
            x: size as usize - 1,
            y: (size as usize - 1) / 2,
        }, // left
        Point {
            x: 0,
            y: (size as usize - 1) / 2,
        }, // right
    ]
    .map(|point| world.reached_from_point(vec![point], size - 1, false) as u64)
    .iter()
    .sum();

    let point_up_left = Point { x: 0, y: 0 };
    let point_up_right = Point {
        x: size as usize - 1,
        y: 0,
    };

    let point_down_left = Point {
        x: 0,
        y: size as usize - 1,
    };
    let point_down_right = Point {
        x: size as usize - 1,
        y: size as usize - 1,
    };

    let one_quarter_filled_points: u64 = [
        point_up_left,
        point_up_right,
        point_down_left,
        point_down_right,
    ]
    .map(|point| world.reached_from_point(vec![point], ((size - 1) / 2) - 1, true) as u64)
    .iter()
    .sum();

    let three_quarter_filled_points: u64 = [
        point_up_left,
        point_up_right,
        point_down_left,
        point_down_right,
    ]
    .map(|point| world.reached_from_point(vec![point], (size - 1) / 2 + size - 1, false) as u64)
    .iter()
    .sum();

    let completely_filled_even =
        world.reached_from_point(vec![world.start], world.size.width, false) as u64;
    let completely_filled_odd =
        world.reached_from_point(vec![world.start], world.size.width, true) as u64;

    let max_number_of_completed_blocks_on_row = (blocks_in_between - 1) * 2 + 1;

    let max_even_rows = max_number_of_completed_blocks_on_row / 2;
    let max_odd_rows = max_number_of_completed_blocks_on_row / 2 + 1;

    let filled_odd_blocks = max_odd_rows + triangle_number(max_odd_rows - 1) * 2;
    let filled_even_blocks = max_even_rows + triangle_number(max_even_rows - 1) * 2;

    let number_of_one_quarter_sets = blocks_in_between;
    let number_of_three_quarter_sets = blocks_in_between - 1;

    (completely_filled_even * filled_even_blocks
        + completely_filled_odd * filled_odd_blocks
        + points_on_axis
        + one_quarter_filled_points * number_of_one_quarter_sets
        + three_quarter_filled_points * number_of_three_quarter_sets)
        .to_string()
}

fn triangle_number(number: u64) -> u64 {
    (number * (number + 1)) / 2
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
        self.reached_from_point(vec![self.start], number_of_steps, number_of_steps % 2 == 0)
    }

    fn reached_from_point(
        &self,
        from: Vec<Point>,
        number_of_steps: usize,
        only_odd: bool,
    ) -> usize {
        let mut reached: HashSet<Point> = HashSet::from_iter(from);

        (reached.len()..=number_of_steps).for_each(|_| {
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
                        .filter_map(|direction| self.walk(point, &direction))
                        .collect::<Vec<Point>>()
                    })
                    .collect::<Vec<Point>>(),
            );
        });

        let filtered: HashSet<Point> = match only_odd {
            true => reached
                .into_iter()
                .filter(|reached_point| {
                    match mod_pos(reached_point.y, 2) == mod_pos(self.start.y, 2) {
                        true => mod_pos(reached_point.x, 2) == mod_pos(self.start.x, 2),
                        false => mod_pos(reached_point.x, 2) != mod_pos(self.start.x, 2),
                    }
                })
                .collect(),
            false => reached
                .into_iter()
                .filter(|reached_point| mod_pos(reached_point.y, 2) != mod_pos(reached_point.x, 2))
                .collect(),
        };

        filtered.len()
    }

    fn blocked(&self, point: &Point) -> bool {
        self.map.contains(self.index_on_map(point))
    }

    fn index_on_map(&self, point: &Point) -> usize {
        let on_map = self.point_on_map(point);

        on_map.y * self.size.width + on_map.x
    }

    fn point_on_map(&self, point: &Point) -> Point {
        Point {
            x: mod_pos(point.x, self.size.width),
            y: mod_pos(point.y, self.size.width),
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
        .and_then(|point| match self.blocked(&point) {
            true => None,
            false => Some(point),
        })
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Size {
    width: usize,
    height: usize,
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn mod_pos(value: usize, division: usize) -> usize {
    ((value % division) + division) % division
}
