use std::fs;
use std::collections::HashMap;
use std::io;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut reports = 0;
    for line in data.lines() {
        let mut v: Vec<i32> = vec![];
        v.extend(line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()));
    
        let differences: Vec<i32> = v.windows(2)
            .map(|p| p[1] - p[0]) // Subtract consecutive numbers
            .collect();
        
        let strictly_decreasing = differences.iter().all(|&d| d < 0 && d.abs() <= 3);
        let strictly_increasing = differences.iter().all(|&d| d > 0 && d.abs() <= 3);
        if strictly_decreasing == true {
            println!("{strictly_decreasing}");
            println!("{:?}", v);
            reports+=1;
        } else if strictly_increasing == true {
            println!("{strictly_increasing}");
            println!("{:?}", v);
            reports+=1;
        }
    }
    println!("{reports}");
    // let y=5;
    // let z=23;
    // let result = (y*x)/x;

}
