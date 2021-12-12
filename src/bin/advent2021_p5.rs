use std::error::Error;

use challenges::read_lines;

#[macro_use]
extern crate scan_fmt;

fn part1() -> Result<(), Box<dyn Error>> {
    let size = 1000;
    let mut grid = vec![0; size * size];
    if let Ok(lines) = read_lines("./inputs/advent2021_p5") {
        for line in lines {
            let line = &line?;
            let (x1, y1, x2, y2) = scan_fmt!(line, "{},{} -> {},{}", usize, usize, usize, usize)?;
            if x1 == x2 {
                let min = y1.min(y2);
                let max = y1.max(y2);
                for i in min..=max {
                    grid[i * size + x1] += 1;
                }
            } else if y1 == y2 {
                let min = x1.min(x2);
                let max = x1.max(x2);
                for i in min..=max {
                    grid[y1 * size + i] += 1;
                }
            }
        }
    }

    dbg!(grid.iter().filter(|c| **c > 1).count());

    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let size = 1000;
    let mut grid = vec![0; size * size];
    if let Ok(lines) = read_lines("./inputs/advent2021_p5") {
        for line in lines {
            let line = &line?;
            let (x1, y1, x2, y2) = scan_fmt!(line, "{},{} -> {},{}", usize, usize, usize, usize)?;
            let minx = x1.min(x2);
            let maxx = x1.max(x2);
            let miny = y1.min(y2);
            let maxy = y1.max(y2);
            if x1 == x2 {
                for i in miny..=maxy {
                    grid[i * size + x1] += 1;
                }
            } else if y1 == y2 {
                for i in minx..=maxx {
                    grid[y1 * size + i] += 1;
                }
            } else {
                if (x1 < x2 && y1 < y2) || (x2 < x1 && y2 < y1) {
                    for i in 0..=maxx - minx {
                        grid[(miny + i) * size + minx + i] += 1;
                    }
                } else {
                    for i in 0..=maxx - minx {
                        grid[(maxy - i) * size + minx + i] += 1;
                    }
                }
            }
        }
    }

    dbg!(grid.iter().filter(|c| **c > 1).count());

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
