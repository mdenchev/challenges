#![feature(drain_filter)]
use std::error::Error;

use challenges::read_lines;

fn part1() -> Result<(), Box<dyn Error>> {
    let mut bit_grid = Vec::<Vec<i32>>::new();
    if let Ok(lines) = read_lines("./inputs/advent2021_p3") {
        for line in lines {
            let line = line?;
            for (idx, ch) in line.chars().enumerate() {
                if bit_grid.len() <= idx {
                    bit_grid.push(vec![]);
                }
                bit_grid[idx].push(ch.to_digit(10).unwrap() as i32);
            }
        }
    }
    let report: Vec<i32> = bit_grid
        .iter()
        .map(|row| {
            if row.iter().sum::<i32>() * 2 > row.len() as i32 {
                1
            } else {
                0
            }
        })
        .collect();
    let mut gamma_rate = 0;
    for bit in report.iter() {
        gamma_rate <<= 1;
        gamma_rate += bit;
    }
    let mut epsilon_rate = 0;
    for bit in report.iter() {
        epsilon_rate <<= 1;
        epsilon_rate += (bit - 1).abs();
    }
    dbg!(gamma_rate * epsilon_rate);

    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut reports: Vec<Vec<i32>> = vec![];
    if let Ok(lines) = read_lines("./inputs/advent2021_p3") {
        for line in lines {
            let line = line?;
            reports.push(
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as i32)
                    .collect(),
            );
        }
    }

    let oxygen_generator_rating = {
        let mut oxygen_generator_rating_candidates = reports.clone();
        let mut idx = 0;
        while oxygen_generator_rating_candidates.len() > 1 {
            let num_ones: i32 = oxygen_generator_rating_candidates
                .iter()
                .map(|c| c[idx])
                .sum();
            let most_common = if num_ones * 2 >= oxygen_generator_rating_candidates.len() as i32 {
                1
            } else {
                0
            };
            let _ = oxygen_generator_rating_candidates
                .drain_filter::<_>(|report| report[idx] != most_common)
                .collect::<Vec<Vec<i32>>>();
            idx += 1;
        }
        let bits = oxygen_generator_rating_candidates.pop().unwrap();

        let mut rating = 0;
        for bit in bits.iter() {
            rating <<= 1;
            rating += bit;
        }
        rating
    };

    let co2_scrubber_rating = {
        let mut co2_scrubber_rating_candidates = reports.clone();
        let mut idx = 0;
        while co2_scrubber_rating_candidates.len() > 1 {
            let num_ones: i32 = co2_scrubber_rating_candidates.iter().map(|c| c[idx]).sum();
            let most_common = if num_ones * 2 >= co2_scrubber_rating_candidates.len() as i32 {
                1
            } else {
                0
            };
            let _ = co2_scrubber_rating_candidates
                .drain_filter::<_>(|report| report[idx] == most_common)
                .collect::<Vec<Vec<i32>>>();
            idx += 1;
        }
        let bits = co2_scrubber_rating_candidates.pop().unwrap();

        let mut rating = 0;
        for bit in bits.iter() {
            rating <<= 1;
            rating += bit;
        }
        rating
    };

    dbg!(oxygen_generator_rating, co2_scrubber_rating);
    dbg!(oxygen_generator_rating * co2_scrubber_rating);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
