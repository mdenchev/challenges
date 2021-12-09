#![feature(drain_filter)]
use std::error::Error;

use challenges::read_lines;

#[derive(Debug, Default)]
struct BingoBoard {
    board: Vec<(bool, i32)>,
}

impl BingoBoard {
    fn push(&mut self, line: Vec<(bool, i32)>) {
        self.board.extend(line);
    }

    fn try_mark_num(&mut self, num: i32) {
        for (marked, this_num) in self.board.iter_mut() {
            if *this_num == num {
                *marked = true;
            }
        }
    }

    fn has_won(&self) -> bool {
        // row check
        if self
            .board
            .chunks(5)
            .any(|row| row.iter().all(|(marked, _)| *marked))
        {
            return true;
        }
        // col check
        if (0..5).into_iter().any(|start| {
            self.board[start..]
                .iter()
                .step_by(5)
                .all(|(marked, _)| *marked)
        }) {
            return true;
        }
        false
    }

    fn unmarked_sum(&self) -> i32 {
        self.board
            .iter()
            .map(|(marked, num)| if *marked { 0 } else { *num })
            .sum()
    }
}

fn part1() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("./inputs/advent2021_p4")?;
    let first_line = lines.next().unwrap()?;
    let order: Vec<i32> = first_line
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = vec![];

    while let Some(Ok(_)) = lines.next() {
        let mut board = BingoBoard::default();
        for _ in 0..5 {
            if let Some(Ok(line)) = lines.next() {
                board.push(
                    line.split_whitespace()
                        .map(|num| (false, num.parse::<i32>().unwrap()))
                        .collect(),
                );
            }
        }
        boards.push(board);
    }

    'outer: for num in order {
        for board in boards.iter_mut() {
            board.try_mark_num(num);
        }

        for board in boards.iter() {
            if board.has_won() {
                dbg!(num * board.unmarked_sum());
                break 'outer;
            }
        }
    }

    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("./inputs/advent2021_p4")?;
    let first_line = lines.next().unwrap()?;
    let order: Vec<i32> = first_line
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = vec![];

    while let Some(Ok(_)) = lines.next() {
        let mut board = BingoBoard::default();
        for _ in 0..5 {
            if let Some(Ok(line)) = lines.next() {
                board.push(
                    line.split_whitespace()
                        .map(|num| (false, num.parse::<i32>().unwrap()))
                        .collect(),
                );
            }
        }
        boards.push(board);
    }

    'outer: for num in order {
        for board in boards.iter_mut() {
            board.try_mark_num(num);
        }

        let mut i = 0;
        while i < boards.len() {
            let board = &boards[i];
            if board.has_won() {
                if boards.len() > 1 {
                    boards.remove(i);
                } else {
                    dbg!(num * board.unmarked_sum());
                    break 'outer;
                }
            } else {
                i += 1;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
