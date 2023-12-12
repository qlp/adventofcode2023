use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::ops::BitXor;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    // print_answer("one (example)", &one(EXAMPLE), "21");
    print_answer("one", &one(INPUT), "");
    // print_answer("two (example)", &two(EXAMPLE), "");
    // print_answer("two (example)", &two(INPUT), "");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let world = World::parse(input);

    // println!("{world}");
    //
    // world.records.iter().for_each(|r| {
    //     let extra_length = r.extra_length();
    //     println!("{r} {extra_length}");
    //
    //     r.candidates().iter().for_each(|c| {
    //         println!("{:#b}", c);
    //     })
    // });
    //
    // let max_extra_length_record = world
    //     .records
    //     .iter()
    //     .max_by_key(|r| r.extra_length())
    //     .expect("at least one");
    // let max_extra_length = max_extra_length_record.extra_length();
    //
    // println!("max: {max_extra_length}");
    //
    // let combinations: u32 = world
    //     .records
    //     .iter()
    //     .map(|r| 2u32.pow(r.extra_length()))
    //     .sum();
    //
    // println!("total: {combinations}");

    world
        .records
        .iter()
        .map(|r| r.candidates().iter().len() as u32)
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    String::new()
}

struct World {
    records: Vec<Record>,
}

impl World {
    fn parse(input: &str) -> Self {
        Self {
            records: input.lines().map(Record::parse).collect(),
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.records.iter().for_each(|r| {
            r.fmt(f);
            f.write_char('\n');
        });

        Ok(())
    }
}

struct Record {
    length: u32,
    has_spring: u32,
    has_no_spring: u32,
    groups: Vec<u32>,
}

impl Record {
    fn parse(input: &str) -> Self {
        let (springs, groups) = input.split_once(' ').expect("space");

        let length = springs.len() as u32;
        let mut has_spring = 0u32;
        let mut has_no_spring = 0u32;
        (0..length).for_each(|i| {
            let bit = 2u32.pow(length - i - 1);
            match springs.chars().nth(i as usize).expect("char at index") {
                '#' => has_spring |= bit,
                '.' => has_no_spring |= bit,
                '?' => {}
                _ => panic!("unexpect char"),
            };
        });

        let groups: Vec<u32> = groups
            .split(',')
            .map(|group| group.parse().expect("number"))
            .collect();

        Self {
            length,
            has_spring,
            has_no_spring,
            groups,
        }
    }

    fn extra_length(&self) -> u32 {
        let length_of_all_groups = self.groups.iter().sum::<u32>();
        let minimum_space_between_groups = self.groups.len() as u32 - 1;

        self.length - length_of_all_groups - minimum_space_between_groups
    }

    fn candidates(&self) -> HashSet<u32> {
        let space: u32 = self.length - self.groups.iter().sum::<u32>();
        let number_of_slots = self.groups.len() as u32 + 1;

        let slots = self.slots(space, number_of_slots, true);

        slots
            .iter()
            .map(|slot| {
                // println!("{:?}", slot);

                self.groups
                    .iter()
                    .enumerate()
                    .fold(0u32, |acc, (index, group)| {
                        let space: u32 = (0..=index).map(|i| slot[i]).sum();
                        let previous_groups: u32 = (0..index).map(|i| self.groups[i]).sum();
                        let start = space + previous_groups;
                        let end = start + group - 1;

                        // println!("{index} {space} {group} {previous_groups} {start} {end}");

                        (start..=end).fold(acc, |acc, bit_index| {
                            // println!("  {bit_index}");
                            let bit_mask = 2u32.pow(self.length - bit_index - 1);
                            // println!("  {:#b}", bit_mask);

                            acc | bit_mask
                        })
                    })
            })
            .filter(|c| {
                let result = self.has_spring & c == self.has_spring;

                // println!("  {:0>32b}", self.has_spring);
                // println!("  {:0>32b}", c);
                // println!("  {:0>32b}", self.has_spring & c);
                // println!("-- ({})", result);

                result
            })
            .filter(|c| {
                let result = self.has_no_spring | !c == !c;

                // println!("  {:0>32b}", self.has_no_spring);
                // println!("  {:0>32b}", !c);
                // println!("  {:0>32b}", self.has_no_spring | !c);
                // println!(">> ({})", result);

                result
            })
            .collect()
    }

    fn slots(&self, space: u32, slots: u32, first: bool) -> Vec<Vec<u32>> {
        let start = match first {
            true => 0,
            false => 1,
        };

        match slots {
            1 => (0..=space).map(|l| vec![l]).collect::<Vec<Vec<u32>>>(),
            _ => (start..=space)
                .flat_map(|l| {
                    let mut tails = self.slots(space - l, slots - 1, false);

                    tails.iter_mut().for_each(|tail| tail.insert(0, l));

                    tails
                })
                .collect::<Vec<Vec<u32>>>(),
        }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.length).rev().for_each(|i| {
            let bit = 2u32.pow(i);
            let has_spring = self.has_spring & bit > 0;
            let has_no_spring = self.has_no_spring & bit > 0;
            match (has_spring, has_no_spring) {
                (false, false) => f.write_char('?'),
                (false, true) => f.write_char('.'),
                (true, false) => f.write_char('#'),
                _ => panic!("unexpected"),
            };
        });

        f.write_char(' ');
        f.write_str(
            self.groups
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .as_str(),
        )?;

        Ok(())
    }
}
