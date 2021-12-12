#![feature(drain_filter)]
use std::error::Error;

use challenges::read_lines;

fn part1() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("./inputs/advent2021_p6")?;
    let mut fish: Vec<i32> = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    for _ in 0..80 {
        for fi in 0..fish.len() {
            fish[fi] -= 1;
            if fish[fi] == -1 {
                fish[fi] = 6;
                fish.push(8);
            }
        }
    }
    dbg!(fish.len());

    Ok(())
}

fn part2_original() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("./inputs/advent2021_p6")?;
    let mut fish: Vec<i32> = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let data: Vec<i64> = (0..9)
        .into_iter()
        .map(|n| {
            let mut fish = vec![n];
            for _ in 0..128 {
                for fi in 0..fish.len() {
                    fish[fi] -= 1;
                    if fish[fi] == -1 {
                        fish[fi] = 6;
                        fish.push(8);
                    }
                }
            }
            fish.len() as i64
        })
        .collect();

    for _ in 0..128 {
        for fi in 0..fish.len() {
            fish[fi] -= 1;
            if fish[fi] == -1 {
                fish[fi] = 6;
                fish.push(8);
            }
        }
    }
    dbg!(fish.iter().map(|f| data[*f as usize]).sum::<i64>());

    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("./inputs/advent2021_p6")?;
    let fish: Vec<usize> = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut buckets = vec![0u128; 9];
    fish.iter().for_each(|f| buckets[*f] += 1);

    let days = 256;
    for _ in 0..days {
        let births = buckets.remove(0);
        buckets.push(births);
        buckets[6] += births;
    }
    dbg!(buckets.iter().sum::<u128>());

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
