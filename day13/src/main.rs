use std::fmt::{Display, Formatter, Write};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "405");
    print_answer("one", &one(INPUT), "32035");
    // print_answer("two (example)", &two(EXAMPLE), "525152");
    // print_answer("two", &two(INPUT), "1088006519007");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    World::parse(input)
        .fields
        .iter()
        .map(|f| f.summary())
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    String::new()
}

struct World {
    fields: Vec<Field>,
}

impl World {
    fn parse(input: &str) -> Self {
        Self {
            fields: input.split("\n\n").map(|str| Field::parse(str)).collect(),
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fields.iter().for_each(|field| {
            field.fmt(f);
            f.write_char('\n');
        });

        Ok(())
    }
}

struct Field {
    width: usize,
    rows: Vec<u32>,
}

impl Field {
    fn parse(input: &str) -> Self {
        Self {
            width: input.lines().next().expect("one").len(),
            rows: input
                .lines()
                .map(|line| {
                    line.chars().fold(0, |acc, c| {
                        let bit = match c {
                            '#' => 1,
                            '.' => 0,
                            _ => panic!("unexpected char"),
                        };

                        acc << 1 | bit
                    })
                })
                .collect(),
        }
    }

    fn summary(&self) -> u32 {
        match Field::reflection_at(&self.rows) {
            None => Field::reflection_at(&self.transposed().rows).expect("a reflection") + 1,
            Some(index) => (index + 1) * 100,
        }
    }

    fn reflection_at(rows: &Vec<u32>) -> Option<u32> {
        let len = rows.len();
        (0usize..len - 1)
            .find(|i| {
                let rows_to_check = (i + 1).min(len - i - 1);

                (0..rows_to_check).all(|row| {
                    let one_index = i - row;
                    let other_index = i + 1 + row;

                    let one = rows[one_index];
                    let other = rows[other_index];

                    one == other
                })
            })
            .map(|i| i as u32)
    }

    fn transposed(&self) -> Self {
        Self {
            width: self.rows.len(),
            rows: (0..self.width)
                .map(|new_row_index| {
                    let read_mask = 2u32.pow((self.width - new_row_index - 1) as u32);

                    self.rows.iter().fold(0u32, |acc, row| {
                        let bit = match row & read_mask == 0 {
                            true => 0,
                            false => 1,
                        };
                        acc << 1 | bit
                    })
                })
                .collect(),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rows.iter().for_each(|row| {
            (0..self.width).for_each(|column| {
                let mask = 1u32 << (self.width - column - 1);
                let rock = row & mask != 0;
                let char = match rock {
                    true => '#',
                    false => '.',
                };

                f.write_char(char);
            });
            f.write_char('\n');
        });

        Ok(())
    }
}
