use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";

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
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn from_pos(from: (i32, i32), to: (i32, i32)) -> Option<Direction> {
        let dx = to.0 - from.0 / i32::abs(to.0 - from.0);
        let dy = to.1 - from.1 / i32::abs(to.1 - from.1);
        match (dx, dy) {
            (-1, 0) => Some(Direction::West),
            (1, 0) => Some(Direction::East),
            (0, -1) => Some(Direction::North),
            (0, 1) => Some(Direction::South),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Pipe(Direction, Direction);

impl Pipe {
    pub fn from_char(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Pipe(Direction::North, Direction::South)),
            '-' => Some(Pipe(Direction::West, Direction::East)),
            'L' => Some(Pipe(Direction::North, Direction::East)),
            'J' => Some(Pipe(Direction::West, Direction::North)),
            '7' => Some(Pipe(Direction::West, Direction::South)),
            'F' => Some(Pipe(Direction::South, Direction::East)),
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

    let mut stile = Tile::Animal;
    let v = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (i, (x1, y1)) in v.iter().enumerate() {
        let x1 = sx + x1;
        let y1 = sy + y1;
        if x1 < 0 || x1 >= tiles.len() as i32 || y1 < 0 || y1 >= tiles[0].len() as i32 {
            continue;
        }
        for (x2, y2) in &v[i + 1..] {
            let x2 = sx + x2;
            let y2 = sy + y2;
            if x2 < 0 || x2 >= tiles.len() as i32 || y2 < 0 || y2 >= tiles[0].len() as i32 {
                continue;
            }
        }
    }

    Ok(Map { tiles, sx, sy })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
