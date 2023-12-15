const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "1320");
    print_answer("one", &one(INPUT), "");
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
    input
        .split(',')
        .map(|hash| {
            dbg!(hash
                .chars()
                .fold(0u32, |acc, c| (acc + c as u8 as u32) * 17 % 256))
        })
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    String::new()
}
