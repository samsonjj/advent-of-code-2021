use std::num::ParseIntError;
use std::error::Error;

const INPUT: &str = include_str!("input.txt");

fn parse(s: &str) -> Result<i32, ParseIntError> {
    Ok(s.parse::<i32>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut iter = INPUT.lines();
    let mut last = parse(iter.next().unwrap())?;
    let mut sum = 0;
    for line in iter {
        let num = parse(line)?;
        if num > last {
            sum += 1; 
        }
        last = num;
    }

    println!("{}", sum);

    let nums = INPUT.lines().map(|line| parse(line).unwrap()).collect::<Vec<i32>>();
    let mut window = nums[0] + nums[1] + nums[2];
    let mut sum = 0;
    for i in 3..nums.len() {
        let num = window - nums[i-3] + nums[i];
        if num > window {
            sum += 1;
        }
        window = num;
    }

    println!("{}", sum);

    Ok(())
}
