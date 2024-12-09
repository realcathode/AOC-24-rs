use regex::Regex;
use std::fs;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"do\(\)|don't\(\)|(mul\([0-9]+,[0-9]+\))").unwrap();
    let data = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut filtered = vec![];
        
    let mut collecting = true;
    for line in data.lines() {
        for m in re.find_iter(line) {
            let match_str = m.as_str();
            if match_str == "do()" {
                collecting = true;
            } else if match_str == "don't()" {
                collecting = false;
            } else if collecting == true {
                filtered.push(match_str);
            }
        }
    }
    let sum: i32 = filtered.iter()
        .map(|s| {
            let nums: Vec<i32> = s[4..s.len() - 1]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
                nums[0] * nums[1]
        }).sum();



    println!("{:?}", sum);
    Ok(())
}