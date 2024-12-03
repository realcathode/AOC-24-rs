use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file");
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
    first_col.sort();
    second_col.sort();
    
    let mut len = 0;
    for (f, s) in first_col.iter().zip(second_col.iter()) {
        len += (f-s).abs();
    }
    
    println!("{}", len); 
}
