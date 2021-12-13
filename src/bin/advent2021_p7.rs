#![feature(drain_filter)]
use std::error::Error;

use challenges::read_lines;

fn part1(crabs: Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let dist_to = |i: usize| move |c: &usize| (*c as i32 - i as i32).abs();
    let fuel = (min..=max)
        .into_iter()
        .map(|i| crabs.iter().map(dist_to(i)).sum::<i32>())
        .min()
        .unwrap() as usize;
    Ok(fuel)
}

fn part2(crabs: Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let dist_to = |i: usize| {
        move |c: &usize| {
            let n = (*c as i32 - i as i32).abs();
            n * (n + 1) / 2
        }
    };
    let fuel = (min..=max)
        .into_iter()
        .map(|i| crabs.iter().map(dist_to(i)).sum::<i32>())
        .min()
        .unwrap() as usize;
    Ok(fuel)
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = read_lines("./inputs/advent2021_p7")
        .into_iter()
        .next()
        .unwrap();
    let crabs: Vec<usize> = inp
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    println!("{}", part1(crabs.clone())?);
    println!("{}", part2(crabs)?);
    Ok(())
}
