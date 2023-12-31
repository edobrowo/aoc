use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";

    let mut data = String::new();
    let mut file = File::open(INPUT_PATH)?;
    file.read_to_string(&mut data)?;

    let mut lines = data.lines();

    let time_str = lines.next().unwrap().strip_prefix("Time:").unwrap().trim();
    let distance_str = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim();

    let distances = distance_str
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;
    let times = time_str
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    // It's quadratic but I'll just check all of them because laziness
    let sol_a: u32 = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| (0..=*t).filter(|x| *t * x - x * x > *d).count() as u32)
        .product();
    println!("{sol_a}");

    // Ok fine I won't be lazy anymore -_-
    let big_time: u64 = time_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let big_distance: u64 = distance_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    // lol
    let mut l = 0;
    for x in 0..=big_time {
        if big_time * x - x * x > big_distance {
            l = x;
            break;
        }
    }
    let mut r = 0;
    for x in (0..=big_time).rev() {
        if big_time * x - x * x > big_distance {
            r = x;
            break;
        }
    }
    let sol_b = r - l + 1;
    println!("{sol_b}");

    Ok(())
}
