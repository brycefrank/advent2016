use std::io::{BufReader, BufRead};
use std::fs::File;

struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn validate_next(&self, nxt_row: i32, nxt_col: i32, keys: &Vec<Vec<char>>) -> bool {
        if nxt_col < 0 || nxt_col > 4 || nxt_row < 0 || nxt_row > 4 {
            false
        } else if keys[nxt_row as usize][nxt_col as usize] == '0' {
            false
        } else {
            true
        }
    }

    fn change_row(&self, change: i32, keys: &Vec<Vec<char>>) -> Pos {
        let nxt_row = self.row + change;
        if self.validate_next(nxt_row, self.col, keys) {
            Pos {
                row: self.row + change,
                col: self.col
            }
        } else {
            Pos {
                row: self.row,
                col: self.col
            }
        }
    }

    fn change_col(&self, change: i32, keys: &Vec<Vec<char>>) -> Pos {
        let nxt_col = self.col + change;
        if self. validate_next(self.row, nxt_col, keys) {
            Pos {
                row: self.row,
                col: self.col + change
            }
        } else {
            Pos {
                row: self.row,
                col: self.col
            }
        }
    }

}


pub fn day_two_part_two(file_path: &str) {
    let keys = vec![vec!['0', '0', '1', '0', '0'],
                    vec!['0', '2', '3', '4', '0'],
                    vec!['5', '6', '7', '8', '9'],
                    vec!['0', 'A', 'B', 'C', '0'],
                    vec!['0', '0', 'D', '0', '0']];

    let file = File::open(file_path).unwrap();
    let mut pos = Pos{ row: 2, col: 0};

    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(string) => string,
            Err(_) => continue,
        };

        for ch in line.chars() {
            pos = match ch {
                'U' => pos.change_row(-1, &keys),
                'D' => pos.change_row( 1, &keys),
                'L' => pos.change_col(-1, &keys),
                'R' => pos.change_col( 1, &keys),
                _ => continue,
            };
        };
        println!("{}", keys[pos.row as usize][pos.col as usize]);
    };
}