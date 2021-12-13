use std::fs;

use anyhow::Result;

fn part1(nums: &[i32]) -> Result<usize> {
    Ok(nums.windows(2).filter(|w| w[1] > w[0]).count())
}

fn part2(nums: &[i32]) -> Result<usize> {
    Ok(nums.windows(4).filter(|w| w[3] > w[0]).count())
}

fn main() -> Result<()> {
    let nums: Vec<i32> = fs::read_to_string("./inputs/advent2021_p1")?
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", part1(&nums)?);
    println!("part 2: {}", part2(&nums)?);
    Ok(())
}
