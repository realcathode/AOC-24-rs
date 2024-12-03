use std::fs;
use std::collections::HashMap;


fn main() {
    let data = fs::read_to_string("../input.txt").expect("Failed to read file");
    let mut first_col = Vec::new();
    let mut second_col = Vec::new();


    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(first), Some(second)) = (parts.get(0), parts.get(1)) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                first_col.push(num1);
                second_col.push(num2);
            }
        }
    }
    let mut right_counts = HashMap::new();

    for &num in &second_col {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for &num in &first_col {
        similarity_score += num * right_counts.get(&num).unwrap_or(&0);
    }

    println!("{}", similarity_score);
}
