use std::fs;
use std::io;
use std::time::Instant;

//https://adventofcode.com/2024/day/1

fn main() -> io::Result<()> {
    let now = Instant::now();

    // Specify the relative path to the file
    let file_path = "./src/puzzle.txt";

    // Read the entire file as a String
    let contents = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = contents.lines().collect();
    // make a new vector to store the lines
    let mut lines_left: Vec<i32> = Vec::new();
    let mut lines_right: Vec<i32> = Vec::new();
    for line in lines {
        let line_arr: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let new_left = line_arr[0].parse::<i32>().unwrap();
        let new_right = line_arr[1].parse::<i32>().unwrap();

        let mut left_insert = false;
        let mut right_insert = false;

        if lines_left.len() == 0 {
            lines_left.push(new_left);
            lines_right.push(new_right);
            left_insert = true;
            right_insert = true;
        }
        for i in 0..lines_left.len() {
            if left_insert && right_insert {
                break;
            }

            if lines_left[i] >= new_left && !left_insert {
                lines_left.insert(0, new_left);
                left_insert = true;
            }
            if lines_right[i] >= new_right && !right_insert {
                lines_right.insert(0, new_right);
                right_insert = true;
            }
            
        }
        if !left_insert {
            lines_left.push(new_left);
        }
        if !right_insert {
            lines_right.push(new_right);
        }

    }
    // Hmmm shouldn't need this
    lines_left.sort();
    lines_right.sort();
    println!("{:?}", lines_right);
    let mut value = 0;
    for (i, left) in lines_left.iter().enumerate() {
        let right = lines_right[i];
        let amount = (left - right).abs();
        value += amount;
    }

    println!("{}", value);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
    // 3574690
}