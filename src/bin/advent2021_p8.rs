#![feature(drain_filter)]
use std::error::Error;

use challenges::read_lines_iter;

type Data = Vec<(Vec<String>, Vec<String>)>;

fn part1(data: &Data) -> Result<usize, Box<dyn Error>> {
    Ok(data
        .iter()
        .flat_map(|(_, o)| o)
        .filter(|v| [2usize, 4, 3, 7].contains(&v.len()))
        .count())
}

fn part2(data: &Data) -> Result<usize, Box<dyn Error>> {
    use std::collections::{HashMap, HashSet};
    fn to_sorted(s: &str) -> String {
        let mut as_vec = s.chars().into_iter().collect::<Vec<char>>();
        as_vec.sort_unstable();
        as_vec.iter().collect()
    }
    let mut sum = 0;
    for row in data {
        let mut len_mapping: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut segments_mapping: HashMap<String, usize> = HashMap::new();
        // known mappings
        for entry in &row.0 {
            let segments: HashSet<char> = entry.chars().collect();
            if let Some(num) = match segments.len() {
                2 => Some(1),
                4 => Some(4),
                3 => Some(7),
                7 => Some(8),
                _ => None,
            } {
                len_mapping.insert(num, segments.clone());
                segments_mapping.insert(to_sorted(entry), num);
            };
        }
        // deduced mappings
        for entry in &row.0 {
            let segments: HashSet<char> = entry.chars().collect();
            match entry.len() {
                6 => {
                    if segments.is_superset(&len_mapping[&4]) {
                        segments_mapping.insert(to_sorted(entry), 9);
                    } else if segments.is_superset(&len_mapping[&1]) {
                        segments_mapping.insert(to_sorted(entry), 0);
                    } else {
                        segments_mapping.insert(to_sorted(entry), 6);
                    }
                }
                5 => {
                    if segments.is_superset(&len_mapping[&1]) {
                        segments_mapping.insert(to_sorted(entry), 3);
                    } else if segments.is_superset(&(&len_mapping[&4] - &len_mapping[&1])) {
                        segments_mapping.insert(to_sorted(entry), 5);
                    } else {
                        segments_mapping.insert(to_sorted(entry), 2);
                    }
                }
                _ => {}
            }
        }

        let output = &row
            .1
            .iter()
            .map(|s| to_sorted(s))
            .fold(0, |acc, x| acc * 10 + segments_mapping[&x]);
        sum += output;
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp: Data = read_lines_iter("./inputs/advent2021_p8")
        .into_iter()
        .map(|line| {
            let (i, o) = line.split_once("|").unwrap();
            let i = i.trim().split(' ').map(|s| s.to_owned()).collect();
            let o = o.trim().split(' ').map(|s| s.to_owned()).collect();
            (i, o)
        })
        .collect();
    println!("{}", part1(&inp)?);
    println!("{}", part2(&inp)?);
    Ok(())
}
