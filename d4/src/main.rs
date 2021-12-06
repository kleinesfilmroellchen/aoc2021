use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Board {
    pub contents: Vec<Vec<u8>>,
    pub marked: Vec<Vec<bool>>,
}

lazy_static! {
    static ref board_regex: Regex =
        Regex::new(r"([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)").unwrap();
}

impl Board {
    fn new(lines: &str) -> Board {
        let mut contents = Vec::<Vec<u8>>::new();
        for line in lines.lines() {
            let mut content_line = Vec::<u8>::new();
            if let Some(groups) = board_regex.captures(line) {
                for i in 1..=5 {
                    content_line.push(u8::from_str(&groups[i]).unwrap());
                }
            }
            contents.push(content_line);
        }
        Board {
            contents: contents.clone(),
            marked: vec![vec![false; contents[0].len()]; contents.len()],
        }
    }

    fn mark(&mut self, number: u8) {
        if let Some((i, j)) = self.contents.iter().enumerate().find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &val)| if val == number { Some(j) } else { None })
                .and_then(|j| Some((i, j)))
        }) {
            self.marked[i][j] = true;
        }
    }

    fn row_solution(&self) -> bool {
        self.marked.iter().any(|row| row.iter().all(|x| *x))
    }
    fn column_solution(&self) -> bool {
        // loop columns
        for i in 0..self.marked[0].len() {
            let mut all_marked = true;
            for j in 0..self.marked.len() {
                if !self.marked[j][i] {
                    all_marked = false;
                    break;
                }
            }
            if all_marked {
                return true;
            }
        }
        false
    }

    fn is_done_p1(&self) -> bool {
        self.row_solution() || self.column_solution()
    }

    fn score(&self, last_num: u8) -> u64 {
        (last_num as u64)
            * self
                .contents
                .iter()
                .flatten()
                .zip(self.marked.iter().flatten())
                .filter_map(|(val, marked)| if !marked { Some(*val as u64) } else { None })
                .sum::<u64>()
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            contents: Vec::default(),
            marked: Vec::default(),
        }
    }
}

fn parse_moves(move_line: String) -> Vec<u8> {
    move_line
        .split(",")
        .map(|mv| u8::from_str(mv).unwrap())
        .collect()
}

fn main() {
    part2();
}

fn part1() {
    let mut file_sections = fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|sr| String::from(sr))
        .collect::<Vec<String>>();
    let moves = parse_moves(file_sections.remove(0));

    let mut boards = file_sections
        .iter()
        .map(|board_spec| Board::new(board_spec))
        .collect::<Vec<Board>>();
    for mov in moves {
        for board in &mut boards {
            board.mark(mov);
        }
        let mut has_winner = false;
        boards
            .iter()
            .filter(|&board| board.is_done_p1())
            .for_each(|winner| {
                has_winner = true;
                println!(
                    "Winner board at {}!\n{:?}\nscore {}",
                    mov,
                    winner,
                    winner.score(mov)
                )
            });
        if has_winner {
            break;
        }
    }
}

fn part2() {
    let mut file_sections = fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|sr| String::from(sr))
        .collect::<Vec<String>>();
    let moves = parse_moves(file_sections.remove(0));

    let mut boards = file_sections
        .iter()
        .map(|board_spec| Board::new(board_spec))
        .collect::<Vec<Board>>();
    let mut last_board: (Board, u8) = (Board::default(), 0);
    for mov in moves {
        for board in &mut boards {
            board.mark(mov);
        }
        last_board = (boards[0].clone(), mov);
        boards.retain(|board| !board.is_done_p1());
        if boards.len() == 0 {
            break;
        }
    }
    println!(
        "Last board is {:#?}\nat move {}, score {}",
        last_board.0,
        last_board.1,
        last_board.0.score(last_board.1)
    );
}
