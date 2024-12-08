use regex::Regex;
use std::fs;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let data = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut results = vec![];
    
    for line in data.lines() {
        for m in re.find_iter(line) {
            results.push(m.as_str());
        }
        println!("{:?}", results);
    }

    let sum: i32 = results.iter()
        .map(|s| {
            // mul(xxxx , xxxx)
            // ~~~^
            let nums: Vec<i32> = s[4..s.len() - 1]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            nums[0] * nums[1]
        }).sum();
    
    println!("{}", sum);
    Ok(())
}