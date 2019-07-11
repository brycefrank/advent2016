use std::io::{BufReader, BufRead};
use std::fs::File;

fn combo_util(arr: &Vec<u32>, r: usize, index: usize, data: &mut Vec<u32>, i:usize, container: &mut Vec<Vec<u32>>, n: usize) {
    if index == r {
        container.push(data.to_vec());

        // TODO find which elements are not in this vector
        // and add to end of data

        return;
    } 
    
    if i < n {
        data[index] = arr[i];
        combo_util(arr, r, index+1, data, i+1, container, n);
        combo_util(arr, r, index, data, i+1, container, n);
    } 
}

pub fn make_combos(n: usize, r: usize) -> Vec<Vec<u32>>{
    // Fill container with three sub-vectors, each with 1's as elements
    let mut container = Vec::new();

    let mut arr: Vec<u32> = Vec::with_capacity(n);
    let mut data: Vec<u32> = vec![0; r];

    for i in 0..n {
        arr.push(i as u32);
    }

    combo_util(&arr, r, 0, &mut data, 0, &mut container, n);

    return container;
}

pub fn day_three(file_path: &str) {
    let file = File::open(file_path).unwrap();

    let combos = make_combos(3, 2);
    let mut num_possible = 0;


    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(string) => string,
            Err(_) => continue,
        };

        let triangle_sides: Vec<u32> = line.split_whitespace()
            .enumerate()
            .map(|(_, e)| {
                let res: u32 = e.parse().unwrap();
                return res;
            })
            .collect(); // Why does this have to be mutable?

        let mut line_sum = 0;

        for i in 0..3 {
            line_sum += triangle_sides[i];
        };

        let mut all_possible = true;

        for combo in &combos {
            let first_side = triangle_sides[combo[0] as usize];
            let second_side = triangle_sides[combo[1] as usize];
            let remainder = line_sum - first_side - second_side;

            if first_side + second_side <= remainder {
                all_possible = false;
                break;
            }
        };

        if all_possible {
            num_possible += 1;
        }
        

    };


    println!("{}", num_possible);
}