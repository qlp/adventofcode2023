use bit_set::BitSet;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_1: &str = include_str!("example-1.txt");
const EXAMPLE_2: &str = include_str!("example-2.txt");
const EXAMPLE_3: &str = include_str!("example-3.txt");
const EXAMPLE_4: &str = include_str!("example-4.txt");

fn main() {
    // test(EXAMPLE_2);
    // print_answer("one (example)", &one(EXAMPLE, 6), "16");
    // print_answer("one", &one(INPUT, 64), "3751");
    // print_answer("two (example) - 6", &two(EXAMPLE_1, 6), "16");
    // print_answer("two (example) - 10", &two(EXAMPLE_1, 10), "50");
    // print_answer("two (example) - 50", &two(EXAMPLE_1, 50), "1594");
    // print_answer("two (example) - 100", &two(EXAMPLE, 100), "6536");
    // print_answer("two (example) - 500", &two(EXAMPLE, 500), "167004");
    // print_answer("two (example) - 1000", &two(EXAMPLE, 1000), "668697");
    // print_answer("two (example) - 5000", &two(EXAMPLE, 5000), "16733044");
    // print_answer(
    //     "two (test 1)",
    //     &two(
    //         EXAMPLE_4,
    //         ((EXAMPLE_4.lines().count() - 1) / 2) + 6 * EXAMPLE_4.lines().count(),
    //     ),
    //     "2228",
    // );
    // print_answer(
    //     "two (test 1)",
    //     &two(
    //         EXAMPLE_2,
    //         ((EXAMPLE_2.lines().count() - 1) / 2) + 3 * EXAMPLE_2.lines().count(),
    //     ),
    //     "2228",
    // );
    // print_answer(
    //     "two (test 2)",
    //     &two(
    //         EXAMPLE_3,
    //         ((EXAMPLE_3.lines().count() - 1) / 2) + 10 * EXAMPLE_3.lines().count(),
    //     ),
    //     "4113",
    // );
    // print_answer(
    //     "two (test 3)",
    //     &two(
    //         INPUT,
    //         ((INPUT.lines().count() - 1) / 2) + 1 * INPUT.lines().count(),
    //     ),
    //     "68157",
    // );
    // print_answer(
    //     "two (test 4)",
    //     &two(
    //         INPUT,
    //         ((INPUT.lines().count() - 1) / 2) + 2 * INPUT.lines().count(),
    //     ),
    //     "189236",
    // );
    print_answer(
        "two",
        &two(INPUT, 26501365),
        "1238814651726318 is too high, 619366400269767 is too low",
    );
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
    let remaining_in_last_block =
        number_of_steps as u64 - (blocks_in_between * size as u64) - steps_to_leave_center;

    println!("{steps_to_leave_center}, {blocks_in_between}, {remaining_in_last_block}");

    let points_on_axis: u64 = [
        Point {
            x: (size as i64 - 1) / 2,
            y: size as i64 - 1,
        }, // up
        Point {
            x: (size as i64 - 1) / 2,
            y: 0,
        }, // down
        Point {
            x: size as i64 - 1,
            y: (size as i64 - 1) / 2,
        }, // left
        Point {
            x: 0,
            y: (size as i64 - 1) / 2,
        }, // right
    ]
    .map(|point| world.reached_from_point(vec![point], size - 1, false, false) as u64)
    .iter()
    .sum();

    let point_up_left = Point { x: 0, y: 0 };
    let point_up_right = Point {
        x: size as i64 - 1,
        y: 0,
    };

    let point_down_left = Point {
        x: 0,
        y: size as i64 - 1,
    };
    let point_down_right = Point {
        x: size as i64 - 1,
        y: size as i64 - 1,
    };

    let one_quarter_filled_points: u64 = [
        point_up_left,
        point_up_right,
        point_down_left,
        point_down_right,
    ]
    .map(|point| world.reached_from_point(vec![point], ((size - 1) / 2) - 1, false, true) as u64)
    .iter()
    .map(|result| dbg!(result))
    .sum();

    let three_quarter_filled_points: u64 = [
        point_up_left,
        point_up_right,
        point_down_left,
        point_down_right,
    ]
    .map(|point| {
        world.reached_from_point(vec![point], (size - 1) / 2 + size - 1, false, false) as u64
    })
    .iter()
    .map(|result| dbg!(result))
    .sum();

    let completely_filled_even =
        dbg!(world.reached_from_point(vec![world.start], world.size.width, false, false)) as u64;
    let completely_filled_odd =
        dbg!(world.reached_from_point(vec![world.start], world.size.width, false, true)) as u64;

    let max_number_of_completed_blocks_on_row = dbg!((blocks_in_between - 1) * 2 + 1);

    let max_even_rows = max_number_of_completed_blocks_on_row / 2;
    let max_odd_rows = max_number_of_completed_blocks_on_row / 2 + 1;

    let filled_odd_blocks = (1..max_odd_rows).fold(max_odd_rows, |acc, n| acc + 2 * n);
    let filled_even_blocks = (1..max_even_rows).fold(max_even_rows, |acc, n| acc + 2 * n);

    // world.reached_from_point(vec![world.start], number_of_steps, true, false);

    let number_of_one_quarter_sets = blocks_in_between;
    let number_of_three_quarter_sets = (blocks_in_between - 1);

    println!("{filled_odd_blocks} {filled_even_blocks} {number_of_one_quarter_sets} {number_of_three_quarter_sets}");
    println!("{completely_filled_even} {one_quarter_filled_points} {three_quarter_filled_points} {points_on_axis}");

    (completely_filled_even * filled_even_blocks
        + completely_filled_odd * filled_odd_blocks
        + points_on_axis
        + one_quarter_filled_points * number_of_one_quarter_sets
        + three_quarter_filled_points * number_of_three_quarter_sets)
        .to_string()
}

fn test(input: &str) {
    let world = World::parse(input);

    (1..=(world.size.width + 2 * world.size.width)).for_each(|steps| {
        println!("{steps}, {}", &world.reached(steps).to_string());
    });
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
        self.reached_from_point(
            vec![self.start],
            number_of_steps,
            true,
            number_of_steps % 2 == 0,
        )
    }

    fn reached_from_point(
        &self,
        from: Vec<Point>,
        number_of_steps: usize,
        unlimited: bool,
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
                        .filter_map(|direction| self.walk(point, &direction, unlimited))
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

        // self.print(filtered.clone());

        filtered.len()
    }

    fn print(&self, reached: HashSet<Point>) {
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
            if mod_pos(y, self.size.height as i64) == 0 {
                println!();
            }
            (min_x..=max_x).for_each(|x| {
                if mod_pos(x, self.size.width as i64) == 0 {
                    print!(" ");
                }
                let point = Point { x, y };
                let did_reach = reached.contains(&point);
                let is_blocked = self.blocked(&point);
                let is_start = point == self.start;

                print!(
                    "{}",
                    match (did_reach, is_blocked, is_start) {
                        (true, false, true) => 'S',
                        (false, false, true) => 's',
                        (true, false, false) => 'O',
                        (false, true, false) => '#',
                        (false, false, false) => '.',
                        _ => panic!("unexpected"),
                    }
                );
            });
            println!();
        });
    }

    fn blocked(&self, point: &Point) -> bool {
        self.map.contains(self.index_on_map(point))
    }

    fn index_on_map(&self, point: &Point) -> usize {
        let on_map = self.point_on_map(point);

        on_map.y as usize * self.size.width + on_map.x as usize
    }

    fn point_on_map(&self, point: &Point) -> Point {
        Point {
            x: mod_pos(point.x, self.size.width as i64),
            y: mod_pos(point.y, self.size.width as i64),
        }
    }

    fn walk(&self, point: &Point, direction: &Direction, unlimited: bool) -> Option<Point> {
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
            false => match unlimited {
                true => Some(candidate),
                false => match candidate.x >= 0
                    && candidate.y >= 0
                    && candidate.x < self.size.width as i64
                    && candidate.y < self.size.height as i64
                {
                    true => Some(candidate),
                    false => None,
                },
            },
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
