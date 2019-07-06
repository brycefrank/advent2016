use std::io::{BufReader, BufRead};
use std::fs::File;

struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn up(&self) -> Pos { 
        Pos {
            row: if self.row > 0 {
                self.row - 1
            } else {
                self.row
            },
            col: self.col}
        }

    fn down(&self) -> Pos { 
        Pos {
            row: if self.row < 2 {
                self.row + 1
            } else {
                self.row
            },
            col: self.col}
        }

    fn left(&self) -> Pos { 
        Pos {
            col: if self.col > 0 {
                self.col - 1
            } else {
                self.col
            },
            row: self.row}
        }

    fn right(&self) -> Pos { 
        Pos {
            col: if self.col < 2 {
                self.col + 1
            } else {
                self.col
            },
            row: self.row}
        }

}


pub fn day_two_part_one(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let key_pad = [[1,2,3], [4,5,6], [7,8,9]];
    let mut pos = Pos{ row: 1, col: 1};

    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(string) => string,
            Err(_) => continue,
        };

        for ch in line.chars() {
            pos = match ch {
                'U' => pos.up(),
                'D' => pos.down(),
                'L' => pos.left(),
                'R' => pos.right(),
                _ => continue,
            
            };

        };
        println!("{}", key_pad[pos.row as usize][pos.col as usize]);
    }
}

pub fn day_two_part_two() {
    let key_pad = [['0', '0', '1', '0', '0'], ['0', '2', '3', '4', '0'], ['5', '6', '7', '8', '9'], ['0', 'A', 'B', 'C', '0'], ['0', '0', 'D', '0', '0']];

}