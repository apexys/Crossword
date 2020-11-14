mod random_iter;
use random_iter::RandomIter;
use smallvec::{smallvec, SmallVec};
#[derive(Copy, Clone, Debug)]
struct PlacedWord<'a>{
    word: &'a [char],
    x: usize,
    y: usize,
    direction_is_right: bool
}

impl<'a> PlacedWord<'a>{
    pub fn new<'b>(word: &'b [char], x: usize, y: usize, direction: bool) -> PlacedWord<'b>{
        PlacedWord{
            word,
            x,
            y,
            direction_is_right: direction
        }
    }

    pub fn letter_at_position(&self, x: usize, y: usize) -> Option<char>{
        if self.direction_is_right{
            if self.y != y {
                None
            }else{
                if self.x > x {
                    None
                }else{
                    if self.x + self.len() < x{
                        None
                    }else{
                        let index = x - self.x;
                        self.word.get(index).map(|c| *c)
                    }
                }
            }
        }else{
            if self.x != x {
                None
            }else{
                if self.y > y {
                    None
                }else{
                    if self.y + self.len() < y{
                        None
                    }else{
                        let index = y - self.y;
                        self.word.get(index).map(|c| *c)
                    }
                }
            }
        }
    }

    pub fn len(&self)->usize{
        self.word.len()
    }

    pub fn intersects_incorrectly<'b>(&'a self, other: PlacedWord<'b>) -> bool{
        match (self.direction_is_right, other.direction_is_right){
            (true, true) => { //Both words point right
                if self.y != other.y { //Words on different lines
                    false
                }else{
                    if self.x + self.len() < other.x{ //Words after each other
                        false
                    }else{
                        for x in self.x .. self.x + self.len(){
                            match (self.letter_at_position(x, self.y), other.letter_at_position(x, self.y)){
                                (Some(a), Some(b)) => {
                                    if a != b {
                                        return true;
                                    }
                                },
                                _ => ()
                            }
                        }
                        false
                    }
                }
            },
            (false, false) => { //Both words point down
                if self.x != other.x { //Words on different columns
                    false
                }else{
                    if self.y + self.len() < other.y{ //Words after each other
                        false
                    }else{
                        for y in self.y .. self.y + self.len(){
                            match (self.letter_at_position(self.x, y), other.letter_at_position(self.x, y)){
                                (Some(a), Some(b)) => {
                                    if a != b {
                                        return true;
                                    }
                                },
                                _ => ()
                            }
                        }
                        false
                    }
                }
            },
            (true, false) => { //One right, one down
                if other.x < self.x || other.x > self.x + self.len(){
                    false
                }else{
                    let x = other.x;
                    let y = self.y;
                    match (self.letter_at_position(x, y), other.letter_at_position(x, y)){
                        (Some(a), Some(b)) => {
                            if a != b {
                                return true;
                            }
                        },
                        _ => ()
                    }
                    false
                }
            },
            (false, true) => { //One right, one down
                if self.x < other.x || self.x > other.x + other.len(){
                    false
                }else{
                    let x = self.x;
                    let y = other.y;
                    match (self.letter_at_position(x, y), other.letter_at_position(x, y)){
                        (Some(a), Some(b)) => {
                            if a != b {
                                return true;
                            }
                        },
                        _ => ()
                    }
                    false
                }
            }
        }

    }
}

#[derive(Clone, Debug)]
struct Board<'a>{
    x_size: usize,
    y_size: usize,
    board: SmallVec<[Option<char>; 512]>,
    placed_words: SmallVec<[PlacedWord<'a>;20]>
}

impl<'a> Board<'a>{
    pub fn new(x_size: usize, y_size: usize) -> Board<'a>{
        Board{
            x_size,
            y_size,
            board: smallvec![None; x_size * y_size],
            placed_words: SmallVec::new()
        }
    }

    pub fn add_word(&mut self, word: PlacedWord<'a>){
        self.placed_words.push(word);
        let word = &self.placed_words[self.placed_words.len() - 1];
        if word.direction_is_right{
            for x in word.x .. word.x + word.len(){
                self.board[x + (self.y_size * word.y)] = word.letter_at_position(x, word.y);
            }
        }else{
            for y in word.y .. word.y + word.len(){
                self.board[word.x + (self.y_size * y)] = word.letter_at_position(word.x, y);
            }
        }
    }

    pub fn word_fits(&self, word: PlacedWord) -> bool{
        if word.x + word.len() > self.x_size || word.y + word.len() > self.y_size{
            false
        }else{
            if self.placed_words.iter().any(|w| w.intersects_incorrectly(word)){
                false
            }else{
                true
            }
        }
    }

    pub fn print(&self) -> Vec<String>{
        let mut result = Vec::new();
        for y in 0 .. self.y_size{
            let line = (0 .. self.x_size).into_iter().map(|x| {
                for w in self.placed_words.iter(){
                    if let Some(char) = w.letter_at_position(x, y){
                        return Some(char)
                    }
                }
                return None
            }).map(|c| c.unwrap_or('_')).collect::<String>();
            result.push(line.clone());
        }
        result
    }
}



fn try_configuration<'a>(words: &[&'a [char]], words_start: usize, board: Board<'a>) -> Option<Board<'a>>{
    if words_start >= words.len(){
        Some(board)
    }else{
        let current_word = words[words_start];
        for x in RandomIter::from(0 .. board.x_size){
            for y in RandomIter::from(0 .. board.y_size){
                for &dir in RandomIter::from(&[false, true][..]){
                    let wordconf = PlacedWord::new(current_word, x,y,dir);
                    if board.word_fits(wordconf){
                        let mut new_board =board.clone();
                        new_board.placed_words.push(wordconf);
                        if let Some(board) = try_configuration(words, words_start + 1, new_board){
                            return Some(board);
                        }
                    }
                }
            }
        }
        None
    }
}

use std::io::BufRead;

fn main() {
    if let Some(filename) = std::env::args().nth(1){
        let handle = std::fs::File::open(filename).expect("Could not open file!");
        let reader = std::io::BufReader::new(handle);
        let words: Vec<String> = reader.lines().map(|l| l.expect("Error reading file")).collect();
        let uppercase: Vec<Vec<char>> = words.into_iter()
            .map(|w| w.trim().to_uppercase())
            .filter(|w|w.len() > 0)
            .map(|w| w.chars().collect::<Vec<_>>())
            .collect();
        let refs: Vec<&[char]> = uppercase.iter().map(|s| s.as_ref()).collect();
        for boardsize in 1 .. 26{
            let board = Board::new(boardsize, boardsize);
            let result = try_configuration(&refs,0, board);
            if let Some(board) = result{
                eprintln!("Solution found for boardsize {}", boardsize);
                let lines = board.print();
                for l in lines{
                    println!("{}", l);
                }
                eprintln!();
                break;
            }else{
                eprintln!("No solution found for boardsize {}", boardsize);
            }
        }
    }
}
