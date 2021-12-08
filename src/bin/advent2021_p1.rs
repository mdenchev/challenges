use std::{collections::VecDeque, error::Error};

use challenges::read_lines;

#[macro_use]
extern crate scan_fmt;

fn part1() -> Result<(), Box<dyn Error>> {
    let mut previous: Option<i32> = None;
    let mut increases = 0;
    if let Ok(lines) = read_lines("./inputs/advent2021_p1") {
        for line in lines {
            let line = &line?;
            let val = Some(scan_fmt!(line, "{}", i32)?);
            if previous.is_some() && val > previous {
                increases += 1;
            }
            previous = val;
        }
    }

    dbg!(increases);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut vals = VecDeque::<i32>::new();
    let mut increases = 0;
    if let Ok(lines) = read_lines("./inputs/advent2021_p1") {
        for line in lines {
            let line = &line?;
            let new = scan_fmt!(line, "{}", i32)?;
            vals.push_back(new);
            if vals.len() == 4 {
                if vals.range(..3).sum::<i32>() < vals.range(1..).sum() {
                    increases += 1;
                }
                vals.pop_front();
            }
        }
    }

    dbg!(increases);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
