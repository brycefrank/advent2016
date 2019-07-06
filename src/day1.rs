use std::fs;

struct Pos {
    facing: char,
    x: i32,
    y: i32,
}

fn split_dir(dir_str: &str) -> Vec<&str> {
    dir_str.split(", ").collect()
}

fn update_facing(rel_dir: &char, face_char: &char) -> char {
    if *rel_dir == 'L' {
        match face_char {
            'N' => 'W',
            'S' => 'E',
            'E' => 'N',
            'W' => 'S',
            _ => 'I', // Is there a better way to handle the catch-all?
        }
    } else {
        match face_char {
            'N' => 'E',
            'S' => 'W',
            'E' => 'S',
            'W' => 'N',
            _ => 'I',
        }
    }
}

fn update_x(pos_x: i32, face_char: char, move_num: i32) -> i32 {
    match face_char {
        'E' => pos_x + move_num,
        'W' => pos_x - move_num,
        _   => pos_x
    }
}

fn update_y(pos_y: i32, face_char: char, move_num: &i32) -> i32 {
    match face_char {
        'N' => pos_y + move_num,
        'S' => pos_y - move_num,
        _   => pos_y
    }
}

fn get_manhattan_dist(pos_x: i32, pos_y: i32, origin_x: i32, origin_y: i32) -> i32 {
    (pos_x - origin_x).abs() + (pos_y - origin_y).abs()
}

pub fn day_one(file_path: &str) {
    let s = fs::read_to_string(file_path)
        .expect("Failed to read file.");

    let split: Vec<&str> = split_dir(&s);

    let mut pos: Pos = Pos {x: 0, y: 0, facing: 'N'};

    for inst in split {
        // Update direction
        let rel_dir = inst.chars().nth(0).unwrap(); // Get first character of the instruction
        pos.facing = update_facing(&rel_dir, &pos.facing);

        // Update position
        let move_num = &inst[1..].parse::<i32>().unwrap();
        pos.x = update_x(pos.x, pos.facing, *move_num);
        pos.y = update_y(pos.y, pos.facing, &move_num);
    }

    let dist = get_manhattan_dist(pos.x, pos.y, 0, 0);
    println!("Day One Solution: {}", dist);
}
