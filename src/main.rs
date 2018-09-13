use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};

const BOARD_WIDTH: usize = 11;
const BOARD_HEIGHT: usize = 11;
const HAND_LIMIT: usize = 7;

fn main() {
    let board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
    let dict = make_dict().unwrap();
    println!("{}", board.to_string());
}

fn make_dict() -> Result<HashSet<String>, IoError> {
    let mut dict: HashSet<String> = HashSet::new();
    let file = File::open("word-list.txt")?;
    for line in BufReader::new(file).lines() {
        dict.insert(line?);
    }
    Ok(dict)
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Placed(char),
    Bonus(Bonus),
}
impl Cell {
    pub fn to_char(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Placed(placed) => *placed,
            Cell::Bonus(bonus) => '*', // FIXME
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Bonus {
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

// Y then X indexing
struct Board([[Cell; BOARD_WIDTH]; BOARD_HEIGHT]);
impl ToString for Board {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for _ in 0..BOARD_WIDTH+2 {
            string.push('-');
        }
        string.push('\n');

        for row in self.0.into_iter() {
            string.push('|');
            for cell in row {
                string.push(cell.to_char());
            }
            string.push('|');
            string.push('\n');
        }

        for _ in 0..BOARD_WIDTH+2 {
            string.push('-');
        }
        string.push('\n');
        string
    }
}
impl Board {
    pub fn adjacent_to_placed_tile(&self, xy: (i32, i32)) -> bool {
        for &y_diff in &[-1,0,1] {
            let y = xy.1 + y_diff;
            if y < 0 || y >= BOARD_HEIGHT as i32 { continue; }

            for &x_diff in &[-1,0,1] {
                if x_diff == 0 && y_diff == 0 { continue; }
                let x = xy.0 + x_diff;
                if x < 0 || x >= BOARD_WIDTH as i32 { continue; }
                
                if let Cell::Placed(_) = self.0[y as usize][x as usize] { return true; }
            }
        }
        false
    }
}

struct Game {
    board: Board,
    hand: Vec<char>,
}
