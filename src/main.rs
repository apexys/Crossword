#![feature(iterator_fold_self)]

mod random_iter;
use random_iter::RandomIter;
use rand::thread_rng;
use rand::seq::SliceRandom;
use smallvec::{smallvec, SmallVec};

#[derive(Clone, Debug)]
struct Board{
    x_size: usize,
    y_size: usize,
    board: SmallVec<[Option<char>; 512]>,
}

impl Board{
    pub fn new(x_size: usize, y_size: usize) -> Board{
        Board{
            x_size,
            y_size,
            board: smallvec![None; x_size * y_size]
        }
    }

    pub fn add_word(&mut self, word: &[char], word_x: usize, word_y: usize, direction_is_right: bool){
        if direction_is_right{
            for x in word_x .. word_x + word.len(){
                self.board[x + (self.y_size * word_y)] = Some(word[x - word_x]);
            }
        }else{
            for y in word_y .. word_y + word.len(){
                self.board[word_x + (self.y_size * y)] = Some(word[y - word_y]);
            }
        }
    }

    pub fn word_fits(&self, word: &[char], word_x: usize, word_y: usize, direction_is_right: bool) -> bool{
        if word_x + word.len() > self.x_size || word_y + word.len() > self.y_size{
            false
        }else{
            if direction_is_right{
                for x in word_x .. word_x + word.len(){
                    match (self.board[x + (self.y_size * word_y)], word.get(x - word_x)){
                        (Some(a), Some(&b)) => {
                            if a != b{
                                return false;
                            }
                        },
                        _ => ()
                    }
                }
                return true;
            }else{
                for y in word_y .. word_y + word.len(){
                    match (self.board[word_x + (self.y_size * y)], word.get(y - word_y)){
                        (Some(a), Some(&b)) => {
                            if a != b{
                                return false;
                            }
                        },
                        _ => ()
                    }
                }
                return true;
            }
        }
    }

    pub fn print(&self) -> Vec<String>{
        let mut result = Vec::new();
        for y in 0 .. self.y_size{
            let line = (0 .. self.x_size).into_iter().map(|x| {
                self.board[x + (self.y_size * y)].unwrap_or('_')
            }).collect::<String>();
            result.push(line.clone());
        }
        result
    }

    pub fn print_filled(&self, charset: &[char]) -> Vec<String>{
        let mut result = Vec::new();
        for y in 0 .. self.y_size{
            let line = (0 .. self.x_size).into_iter().map(|x| {
                self.board[x + (self.y_size * y)].unwrap_or(*charset.choose(&mut thread_rng()).unwrap_or(&'_'))
            }).collect::<String>();
            result.push(line.clone());
        }
        result
    }
}



fn try_configuration(words: &[&[char]], words_start: usize, board: Board) -> Option<Board>{
    if words_start >= words.len(){
        Some(board)
    }else{
        let current_word = words[words_start];
        for x in RandomIter::from(0 .. board.x_size){
            for y in RandomIter::from(0 .. board.y_size){
                for &dir in RandomIter::from(&[false, true][..]){
                    if board.word_fits(current_word, x,y,dir){
                        let mut new_board =board.clone();
                        new_board.add_word(current_word, x, y, dir);
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
        if uppercase.len() < 1{
            eprintln!("No words in file!");
            return;
        }
        let mut min_board_size = uppercase.iter().map(|s| s.len()).max().unwrap_or(1);
        let override_board_size: Option<u8> = std::env::args().nth(2).map(|a| a.parse().ok()).flatten();
        if let Some(override_min_board_size) = override_board_size{
            min_board_size = min_board_size.max(override_min_board_size as usize);
        }
        let mut charset: Vec<char> = uppercase.iter().flatten().copied().collect();
        charset.sort_unstable();
        charset.dedup();
        eprintln!("Minimum board size: {}", min_board_size);
        eprintln!("Charset length: {}", charset.len());
        let refs: Vec<&[char]> = uppercase.iter().map(|s| s.as_ref()).collect();
        for boardsize in min_board_size .. min_board_size * min_board_size{
            let board = Board::new(boardsize, boardsize);
            let result = try_configuration(&refs,0, board);
            if let Some(board) = result{
                eprintln!("Solution found for boardsize {}", boardsize);
                let lines = board.print_filled(&charset[..]);
                for l in lines{
                    println!("{}", l);
                }
                println!();
                let words = uppercase.iter().map(|w| w.iter().collect::<String>()).fold_first(|prev, next| prev + ", " + &next).unwrap();
                println!("Find in this: {}",  words);
                println!();
                println!("Solution:");
                let lines = board.print();
                for l in lines{
                    println!("{}", l);
                }
                break;
            }else{
                eprintln!("No solution found for boardsize {}", boardsize);
            }
        }
    }
}
