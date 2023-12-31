const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "288");
    print_answer("one", &one(INPUT), "1195150");
    print_answer("two (example)", &two(EXAMPLE), "71503");
    print_answer("two", &two(INPUT), "42550411");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    parse_one(input)
        .races
        .iter()
        .map(|r| r.winning_count())
        .reduce(|acc, c| acc * c)
        .expect("at least one")
        .to_string()
}

fn two(input: &str) -> String {
    parse_two(input).winning_count().to_string()
}

fn parse_one(input: &str) -> Races {
    let input: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter(|w| !w.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect();

    Races {
        races: (1..input.get(0).expect("expect 2 rows").len())
            .map(|i| Race {
                time: input
                    .get(0)
                    .expect("expect 2 row (1/2)")
                    .get(i)
                    .expect("expect value")
                    .parse()
                    .expect("expect number"),
                distance: input
                    .get(1)
                    .expect("expect 2 row (2/2)")
                    .get(i)
                    .expect("expect value")
                    .parse()
                    .expect("expect number"),
            })
            .collect(),
    }
}

fn parse_two(input: &str) -> Race {
    let input: Vec<u64> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .filter(|w| !w.is_empty())
                .collect::<Vec<&str>>()
                .join("")
        })
        .map(|s| s.parse().expect("number"))
        .collect();

    Race {
        time: *input.first().expect("time"),
        distance: *input.get(1).expect("distance"),
    }
}

#[derive(Debug, Clone)]
struct Races {
    races: Vec<Race>,
}

#[derive(Debug, Clone)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_count(&self) -> u64 {
        let (from, to) = find_roots(-1_f64, self.time as f64, -(self.distance as f64));

        let to = if to == (to as u64 as f64) {
            to as u64 - 1
        } else {
            to as u64
        };

        to - from as u64
    }
}

fn find_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b * b - 4_f64 * a * c;

    let sqrt_discriminant = discriminant.sqrt();

    let divisor = 2_f64 * a;

    let root1 = (-b + sqrt_discriminant) / divisor;
    let root2 = (-b - sqrt_discriminant) / divisor;

    (root1, root2)
}
