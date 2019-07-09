use std::io::{BufReader, BufRead};
use std::fs::File;

// TODO get from opticrib
fn make_combos(n: i32) {

}

pub fn day_three(file_path: &str) {
    let file = File::open(file_path).unwrap();


    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(string) => string,
            Err(_) => continue,
        };

        println!("{}", line);

    };


}