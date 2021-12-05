use std::fs::File;
use std::collections::HashSet;
use std::io::{
    self,
    Read,
    Error,
    ErrorKind,
    BufReader
};

struct Cell (u32, bool);
impl Cell {
    pub fn new(value: u32) -> Self {
        Self (value, false)
    }

    pub fn marked(&self) -> bool {
        self.1
    }

    pub fn mark(&mut self) {
        self.1 = true;
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}
struct Board {
    cells: Vec<Cell>,
    won: bool
}

impl Board {
    pub fn new(values: Vec<u32>) -> Self {
        let cells = values
            .iter()
            .map(|value| { Cell::new(*value) })
            .collect();
        Self {
            cells: cells,
            won: false
        }
    }

    pub fn attempt_mark(&mut self, input: u32) {
        for cell in &mut self.cells {
            if cell.value() == input {
                cell.mark();
                return;
            }
        }
    }

    pub fn check_bingo(&mut self) -> bool {
        if self.won {
            return true;
        }
        let rows = self.cells.chunks(5); 

        // index * 5 + column

        let mut win = false;
        let mut count = 0;
        for row in rows {
            for cell in row {
                if !cell.marked() {
                    count = 0;
                    break;
                }
                count += 1;
            }
            if count == 5 {
                win = true;
                break;
            }
        }

        for i in 0..5 { // column number
            for j in 0..5 { // index of column
                let index = j * 5 + i;
                if !self.cells[index].marked() {
                    count = 0;
                    break;
                }
                count += 1;
            }
            if count == 5 {
                win = true;
                break;
            }
        }
        self.won = win;
        win
    }

    pub fn score(&self, winning_number: u32) -> u32 {
        let mut score = 0;
        for cell in &self.cells {
            if !cell.marked() {
                score += cell.value();
            }
        }
        score *= winning_number;
        return score;
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    if args.len() == 1 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    let file_contents: String;
    let (inputs, mut boards) = {
        let file_name = args.nth(1).unwrap();
        let file = File::open(file_name)?;
        let mut bufreader = BufReader::new(file);

        let mut buffer = Vec::new();
        bufreader.read_to_end(&mut buffer)?;
        file_contents = String::from_utf8(buffer).unwrap();
        let mut data_iter = file_contents.split("\n\n");
        let inputs = data_iter
            .next()
            .unwrap()
            .split(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut boards_values = Vec::new();
        for data in data_iter {
            let mut numbers = Vec::new();
            for value in data.split_whitespace() {
                let number = value.parse::<u32>().unwrap();
                numbers.push(number);
            }
            boards_values.push(numbers);
        }  
        let boards = boards_values.iter().map(|v| {
            return Board::new(v.to_vec());
        }).collect::<Vec<Board>>();

        (inputs, boards)
    };

    let mut winners = HashSet::new();
    let board_len = boards.len();
    for input in inputs {
        for (index, board) in &mut boards.iter_mut().enumerate() {
            board.attempt_mark(input);
            if board.check_bingo() {
                winners.insert(index);
                println!("win index {} ", index);
                if winners.len() == board_len {
                    let score = board.score(input);
                    println!("Board {} wins last with score {} points!", index + 1, score);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}