use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};

const BOARD_WIDTH: usize = 11;
const BOARD_HEIGHT: usize = 11;
const HAND_LIMIT: usize = 7;

fn main() {
    let board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
    let _dict = make_dict().unwrap();
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
            Cell::Bonus(_) => '*', // FIXME
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
    pub fn all_tiles_connected(&self) -> bool {
        let mut tiles: HashSet<(usize, usize)> = HashSet::new();
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_HEIGHT {
                if let Cell::Placed(_) = self.0[y][x] {
                    tiles.insert((x, y));
                }
            }
        }
        // Get an arbitrary tile
        let option_next = tiles.iter().next().cloned();
        match option_next {
            // No tiles? Trivially connected
            None => return true,
            Some(first_ix) => {
                // This queue contains tiles connected to our original tile
                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                tiles.remove(&first_ix);
                queue.push_front(first_ix);
                for _ in 0..BOARD_WIDTH*BOARD_HEIGHT+1 {
                    let option_front = queue.pop_front();
                    match option_front {
                        // We've run out of connected tiles to visit
                        // So we check if there are any unvisited (unconnected) tiles
                        None => return tiles.is_empty(),
                        Some((x, y)) => {
                            // look at surrounding tiles:
                            //   1) discard any that aren't Placed
                            //   2) remove from `tiles`
                            //   3) any that _were_ in `tiles`, add to `queue`
                            for &(x_diff, y_diff) in &[(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                                if x == 0 && x_diff == -1 { continue; }
                                if x == BOARD_WIDTH && x_diff == 1 { continue; }
                                if y == 0 && y_diff == -1 { continue; }
                                if y == BOARD_HEIGHT && y_diff == 1 { continue; }
                                let new_x = (x as i32 +x_diff) as usize;
                                let new_y = (y as i32 +y_diff) as usize;
                                if let Cell::Placed(_) = self.0[y][x] {
                                    let removed = tiles.remove(&(new_x, new_y));
                                    if removed {
                                        queue.push_front((new_x, new_y));
                                    }
                                }
                            }
                        }
                    }
                }
                panic!("Somehow looped past the whole array")
            }
        }
    }
}

struct Game {
    board: Board,
    hand: Vec<char>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_tiles_connected_empty() {
        let board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
        assert!(board.all_tiles_connected());
    }

    #[test]
    fn all_tiles_connected_single() {
        let mut board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
        board.0[3][4] = Cell::Placed('c');
        assert!(board.all_tiles_connected());
    }

    #[test]
    fn all_tiles_connected_multiple() {
        let mut board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
        board.0[3][4] = Cell::Placed('c');
        board.0[3][5] = Cell::Placed('c');
        board.0[3][6] = Cell::Placed('c');
        board.0[4][5] = Cell::Placed('c');
        assert!(board.all_tiles_connected());
    }

    #[test]
    fn all_tiles_connected_full() {
        let mut board = Board([[Cell::Placed('c'); BOARD_WIDTH]; BOARD_HEIGHT]);
        assert!(board.all_tiles_connected());
    }


    #[test]
    fn all_tiles_connected_edge() {
        let mut board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
        board.0[0][0] = Cell::Placed('c');
        board.0[0][1] = Cell::Placed('c');
        assert!(board.all_tiles_connected());
    }

    #[test]
    fn all_tiles_connected_disconnect() {
        let mut board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
        board.0[3][4] = Cell::Placed('c');
        board.0[3][6] = Cell::Placed('c');
        assert!(!board.all_tiles_connected());
    }

    #[test]
    fn all_tiles_connected_disconnect_diag() {
        let mut board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
        board.0[3][4] = Cell::Placed('c');
        board.0[4][5] = Cell::Placed('c');
        assert!(!board.all_tiles_connected());
    }

    #[test]
    fn all_tiles_connected_disconnect_multiple() {
        let mut board = Board([[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT]);
        board.0[3][4] = Cell::Placed('c');
        board.0[3][5] = Cell::Placed('c');
        board.0[3][1] = Cell::Placed('c');
        board.0[3][2] = Cell::Placed('c');
        assert!(!board.all_tiles_connected());
    }
}
