use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;

// Ridiculously verbose for learning's sake
fn main() {
    const INPUT_PATH: &str = "input.txt";

    let lines = read_lines(INPUT_PATH).unwrap();
    let digits = parse_lines(lines, extract_digits).unwrap();
    let sol_a = solve(&digits);
    println!("{sol_a}");

    // Yes I know there is a better way to do this
    let lines = read_lines(INPUT_PATH).unwrap();
    let digits = parse_lines(lines, extract_digits_all).unwrap();
    let sol_a = solve(&digits);
    println!("{sol_a}");
}

fn solve(digits: &Vec<Digits>) -> u32 {
    digits
        .iter()
        .fold(0, |acc, x| acc + x[0].value() * 10 + x[x.len() - 1].value())
}

#[derive(Clone, Copy)]
struct Digit(u32);

impl Digit {
    pub fn from(d: u32) -> Self {
        assert!(d <= 9, "out of bounds");
        Digit(d)
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub const DIGIT_NAMES: [&'static str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // pub fn parse_char(c: char) -> Result<Self, io::Error> {
    //     let d = c.to_digit(10).ok_or(io::Error::new(
    //         io::ErrorKind::InvalidInput,
    //         "invalid digit name",
    //     ))?;
    //     Ok(Digit(d))
    // }

    // pub fn parse_str(s: &str) -> Result<Self, io::Error> {
    //     let d = Self::DIGIT_NAMES
    //         .iter()
    //         .position(|&dn| dn == s)
    //         .ok_or(io::Error::new(
    //             io::ErrorKind::InvalidInput,
    //             "invalid digit name",
    //         ))?;
    //     Ok(Digit(d as u32))
    // }
}

struct Digits(Vec<Digit>);

impl Digits {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // pub fn push(&mut self, d: Digit) {
    //     self.0.push(d)
    // }
}

impl Index<usize> for Digits {
    type Output = Digit;
    fn index<'a>(&'a self, i: usize) -> &'a Digit {
        &self.0[i]
    }
}

fn extract_digits(s: &str) -> Digits {
    Digits(
        s.chars()
            .filter_map(|x| x.to_digit(10).map(|x| Digit::from(x)))
            .collect(),
    )
}

// https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/01.rs
fn extract_digits_all(s: &str) -> Digits {
    let s = s.as_bytes();
    let v = (0..s.len())
        .filter_map(|i| match s[i] {
            b'0'..=b'9' => Some(Digit((s[i] - b'0') as u32)),
            _ => Digit::DIGIT_NAMES
                .iter()
                .enumerate()
                .find_map(|(namei, name)| {
                    s[i..]
                        .starts_with(name.as_bytes())
                        .then_some(Digit(namei as u32))
                }),
        })
        .collect();
    Digits(v)
}

fn parse_lines<F>(
    lines: io::Lines<io::BufReader<File>>,
    parse_fn: F,
) -> Result<Vec<Digits>, io::Error>
where
    F: Fn(&str) -> Digits,
{
    let lines: Result<Vec<String>, std::io::Error> = lines.collect();
    Ok(lines?.iter().map(|line| parse_fn(&line)).collect())
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
