use std::error::Error;

use challenges::read_lines_iter;

#[macro_use]
extern crate scan_fmt;

fn part1() -> Result<(), Box<dyn Error>> {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in read_lines_iter("./inputs/advent2021_p2") {
        let (dir, amt) = scan_fmt!(&line, "{} {}", String, i32)?;
        match dir.as_str() {
            "forward" => horizontal += amt,
            "down" => depth += amt,
            "up" => depth -= amt,
            _ => unreachable!(),
        }
    }

    dbg!(horizontal * depth);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in read_lines_iter("./inputs/advent2021_p2") {
        let (dir, amt) = scan_fmt!(&line, "{} {}", String, i32)?;
        match dir.as_str() {
            "forward" => {
                horizontal += amt;
                depth += aim * amt;
            }
            "down" => aim += amt,
            "up" => aim -= amt,
            _ => unreachable!(),
        }
    }

    dbg!(horizontal * depth);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
