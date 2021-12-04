#![feature(assert_matches)]

const INPUT: &str = include_str!("input.txt");

use std::collections::HashMap;
use std::str::FromStr;

type Board<T> = [[T; 5]; 5];

#[derive(Clone, Debug)]
pub struct BingoBoard {
    pub numbers: HashMap<i32, (usize, usize)>,
    pub marked: Board<bool>,
}

impl BingoBoard {
    // returns the score if the board has won
    pub fn hit(&mut self, num: i32) -> Option<(usize, usize)> {
        if let Some((i, j)) = self.numbers.get(&num) {
            self.marked[*i][*j] = true;
            return Some((*i, *j));
        }
        None
    }

    pub fn won(&self) -> bool {
        for a in 0..5 {
            let mut row_win = true;
            let mut col_win = true;
            for b in 0..5 {
                if !self.marked[a][b] {
                    row_win = false;
                }
                if !self.marked[b][a] {
                    col_win = false;
                }
            }
            if row_win || col_win {
                return true;
            }
        }
        false
    }

    pub fn score(&self, num: i32) -> i32 {
        self.numbers
            .iter()
            .filter_map(|(num, (i, j))| {
                if !self.marked[*i][*j] {
                    Some(*num)
                } else {
                    None
                }
            })
            .sum::<i32>()
            * num
    }
}

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.marked.iter() {
            for col in row.iter() {
                write!(
                    f,
                    "{}",
                    match col {
                        true => "X",
                        false => " ",
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for BingoBoard {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers: HashMap<i32, (usize, usize)> = HashMap::new();
        for (i, line) in s.lines().enumerate() {
            for (j, token) in line.split_whitespace().enumerate() {
                numbers.insert(token.parse::<i32>()?, (i, j));
            }
        }

        Ok(BingoBoard {
            numbers,
            marked: [[false; 5]; 5],
        })
    }
}

fn main() {
    let mut parts = INPUT.split("\n\n");

    // parse numbers that will be called in order
    let numbers_order_str = parts.next().unwrap();
    let numbers: Vec<i32> = numbers_order_str
        .split(',')
        .flat_map(i32::from_str)
        .collect();

    // parse boards
    let boards_tmp: Vec<BingoBoard> = parts.flat_map(BingoBoard::from_str).collect();
    let mut boards = boards_tmp.clone();

    let mut score = None;
    for num in numbers.iter() {
        for ref mut board in boards.iter_mut() {
            board.hit(*num);
            if board.won() {
                score = Some(board.score(*num));
                break;
            }
        }
        if let Some(_) = score {
            break;
        }
    }

    println!("{}", score.unwrap());

    let mut boards: Vec<BingoBoard> = boards_tmp;
    let mut score = None;

    for num in numbers.iter() {
        if boards.len() > 1 {
            for board in boards.iter_mut() {
                board.hit(*num);
            }
            boards = boards.into_iter().filter(|board| !board.won()).collect();
        } else {
            boards[0].hit(*num);
            if boards[0].won() {
                score = Some(boards[0].score(*num));
                break;
            }
        }
    }

    println!("{}", score.unwrap());
}

mod tests {
    use super::*;

    #[test]
    fn won() {
        let mut bingo_board = BingoBoard::from_str(
            "14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
        )
        .unwrap();

        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(bingo_board.won(), false);
            }
        }
        bingo_board.marked[2][0] = true;
        bingo_board.marked[2][1] = true;
        bingo_board.marked[2][2] = true;
        bingo_board.marked[2][3] = true;
        assert_eq!(bingo_board.won(), false);

        bingo_board.marked[2][4] = true;
        assert_eq!(bingo_board.won(), true);
    }
}
