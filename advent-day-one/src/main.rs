use std::fs;
use std::io;

//https://adventofcode.com/2024/day/1

fn main() -> io::Result<()> {
    // Specify the relative path to the file
    let file_path = "./src/puzzle.txt";

    // Read the entire file as a String
    let contents = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = contents.lines().collect();
    // make a new vector to store the lines
    let mut lines_left: Vec<String> = Vec::new();
    let mut lines_right: Vec<String> = Vec::new();
    for line in lines {
        let line_arr: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        lines_left.push(line_arr[0].to_string());
        lines_right.push(line_arr[1].to_string());
    }
    lines_left.sort();
    lines_right.sort();
    let mut value = 0;
    for (i, left_str) in lines_left.iter().enumerate() {
        println!("{}", left_str);
        let left = left_str.parse::<i32>().unwrap();
        let right_str = lines_right[i].clone();
        println!("{}", right_str);
        let right = right_str.parse::<i32>().unwrap();

        let amount = (left - right).abs();
        value += amount;
    }

    println!("{}", value);
    Ok(())
}