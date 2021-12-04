use std::collections::VecDeque;
use std::error::Error;

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    let mut last = 0;
    for line in INPUT.lines() {
        let num = line.parse::<i32>()?;
        if num > last {
            sum += 1;
        }
        last = num;
    }

    println!("{}", sum);

    let mut sum = 0;
    let mut window: VecDeque<i32> = VecDeque::new();
    let mut iter = INPUT.lines().map(|line| line.parse::<i32>().unwrap());
    window.push_back(iter.next().unwrap());
    window.push_back(iter.next().unwrap());
    window.push_back(iter.next().unwrap());
    for item in iter {
        let prev_sum: i32 = window.iter().sum();
        window.pop_front();
        window.push_back(item);
        let curr_sum: i32 = window.iter().sum();
        if curr_sum > prev_sum {
            sum += 1;
        }
    }

    println!("{}", sum);

    Ok(())
}
