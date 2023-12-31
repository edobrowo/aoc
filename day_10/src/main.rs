use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "example1.txt";

    let lines = read_lines(INPUT_PATH)?;
    let map = parse(lines)?;

    let sol_a = solve_a(&map);
    println!("{sol_a}");

    let sol_b = solve_b(&map);
    println!("{sol_b}");

    Ok(())
}

fn solve_a(map: &Map) -> u32 {
    1
}

fn solve_b(map: &Map) -> u32 {
    2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Pipe {
    n: bool,
    s: bool,
    w: bool,
    e: bool,
}

impl Pipe {
    pub fn from_char(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Pipe {
                n: true,
                s: true,
                w: false,
                e: false,
            }),
            '-' => Some(Pipe {
                n: false,
                s: false,
                w: true,
                e: true,
            }),
            'L' => Some(Pipe {
                n: true,
                s: false,
                w: false,
                e: true,
            }),
            'J' => Some(Pipe {
                n: true,
                s: false,
                w: true,
                e: false,
            }),
            '7' => Some(Pipe {
                n: false,
                s: true,
                w: true,
                e: false,
            }),
            'F' => Some(Pipe {
                n: false,
                s: true,
                w: false,
                e: true,
            }),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Animal,
}

impl Tile {
    pub fn from_char(c: char) -> Option<Tile> {
        match c {
            '|' | '-' | 'L' | 'J' | '7' | 'F' => Some(Tile::Pipe(Pipe::from_char(c).unwrap())),
            '.' => Some(Tile::Ground),
            'S' => Some(Tile::Animal),
            _ => None,
        }
    }

    pub fn pipe(&self) -> Option<Pipe> {
        if let Tile::Pipe(p) = self {
            Some(*p)
        } else {
            None
        }
    }

    pub fn can_connect(&self, other: &Tile) -> bool {
        let f = self.pipe();
        let t = other.pipe();
        if (f.is_none() || t.is_none()) {
            return false;
        }
        let f = self.pipe().unwrap();
        let t = other.pipe().unwrap();
        true
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    sx: i32,
    sy: i32,
}

// impl Map {
//     pub fn next_in_loop(self, from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
//         for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
//             let x = to.0 + x;
//             let y = to.1 + y;
//             if x < 0
//                 || x >= self.tiles.len() as i32
//                 || y < 0
//                 || y >= self.tiles[0].len() as i32
//                 || (x == from.0 && y == from.0)
//             {
//                 continue;
//             }
//             let next = &self.tiles[y as usize][x as usize];
//             if next.pipe().is_none() {
//                 if *next == Tile::Animal {
//                     return (x, y);
//                 }
//                 continue;
//             }
//             let curr = self.tiles[to.1 as usize][to.0 as usize].pipe().unwrap();
//             let next = next.pipe().unwrap();
//             let expected_dir = Direction::from_pos((to.0, to.1), (x, y)).unwrap();
//             if expected_dir == curr.0 && (curr.0 == next.0 || curr.0 == next.1)
//                 || expected_dir == curr.1 && (curr.1 == next.0 || curr.1 == next.1)
//             {
//                 return (x, y);
//             }
//         }

//         (1, 1)
//     }
// }

fn parse(lines: io::Lines<io::BufReader<fs::File>>) -> Result<Map, Box<dyn Error>> {
    let mut tiles = lines
        .map(|line| {
            line?
                .chars()
                .map(|c| Ok(Tile::from_char(c).ok_or("invalid")?))
                .collect::<Result<Vec<Tile>, Box<dyn Error>>>()
        })
        .collect::<Result<Vec<Vec<Tile>>, Box<dyn Error>>>()?;

    let (mut sx, mut sy) = (0, 0);
    for (y, row) in tiles.iter().enumerate() {
        if let Some(x) = row.iter().position(|tile| *tile == Tile::Animal) {
            sx = x as i32;
            sy = y as i32;
            break;
        }
    }

    // Assume S is not at the edge
    let u = &tiles[sy as usize - 1][sx as usize].pipe();
    let d = &tiles[sy as usize + 1][sx as usize].pipe();
    let l = &tiles[sy as usize][sx as usize - 1].pipe();
    let r = &tiles[sy as usize][sx as usize + 1].pipe();

    Ok(Map { tiles, sx, sy })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
