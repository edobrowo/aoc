use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";

    let lines = read_lines(INPUT_PATH)?;
    let mut map = parse(lines)?;

    let (sol_a, pipe) = solve_a(&mut map);
    println!("{sol_a}");

    let sol_b = solve_b(&map, &pipe);
    println!("{sol_b}");

    Ok(())
}

// simplifying - https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/10.rs
// my brain is inadequate
// thank you for indirectly teaching me
fn solve_a(map: &mut Map) -> (u32, HashSet<(usize, usize)>) {
    let hs = "|-LJ7F"
        .chars()
        .find_map(|s| {
            map.tiles[map.srow][map.scol] = connections(s);

            let mut visited = HashSet::new();
            let (mut row, mut col) = (map.srow, map.scol);
            let mut to = map.tiles[row][col].iter().position(|&t| t).unwrap();
            let pipe = loop {
                if !visited.insert((row, col)) {
                    break Some(visited);
                }
                let from = match to {
                    0 => {
                        row -= 1;
                        1
                    }
                    1 => {
                        row += 1;
                        0
                    }
                    2 => {
                        col -= 1;
                        3
                    }
                    3 => {
                        col += 1;
                        2
                    }
                    _ => unreachable!(),
                };
                if !map.tiles[row][col][from] {
                    dbg!(row, col, from, to);
                    break None;
                }
                to = (0..4)
                    .find(|&i| i != from && map.tiles[row][col][i])
                    .unwrap();
            };
            pipe
        })
        .unwrap();
    (hs.len() as u32 / 2, hs)
}

fn solve_b(map: &Map, pipe: &HashSet<(usize, usize)>) -> u32 {
    let mut count = 0;
    for (row, line) in map.tiles.iter().enumerate() {
        let mut inside = false;
        for (col, tile) in line.iter().enumerate() {
            if !pipe.contains(&(row, col)) {
                count += inside as u32;
            } else if tile[0] {
                inside = !inside;
            }
        }
    }
    count
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<[bool; 4]>>,
    srow: usize,
    scol: usize,
}

fn connections(c: char) -> [bool; 4] {
    match c {
        '|' => [true, true, false, false],
        '-' => [false, false, true, true],
        'L' => [true, false, false, true],
        'J' => [true, false, true, false],
        '7' => [false, true, true, false],
        'F' => [false, true, false, true],
        _ => [false, false, false, false],
    }
}

fn parse(lines: io::Lines<io::BufReader<fs::File>>) -> Result<Map, Box<dyn Error>> {
    let mut start = (0, 0);
    let tiles = lines
        .enumerate()
        .map(|(row, line)| {
            Ok(line?
                .chars()
                .enumerate()
                .map(|(col, tile)| {
                    if tile == 'S' {
                        start = (row, col);
                    }
                    connections(tile)
                })
                .collect::<Vec<[bool; 4]>>())
        })
        .collect::<Result<Vec<Vec<[bool; 4]>>, Box<dyn Error>>>()?;

    Ok(Map {
        tiles,
        srow: start.0,
        scol: start.1,
    })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
