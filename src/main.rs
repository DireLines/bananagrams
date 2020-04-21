use ndarray::Array2;
use rand::prelude::*;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{hash_map::DefaultHasher, HashSet},
    env,
    fs::File,
    hash::{Hash, Hasher},
    io::{self, BufRead},
    iter::FromIterator,
    path::Path,
};

#[macro_use]
extern crate ndarray;

//evaluates the type of an expr
//and throws a mismatched type error
macro_rules! typecheck {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        let _: () = $e;
    };
}

fn main() {
    let numargs = env::args().count();
    if numargs < 2 || arg_exists("-help") {
        println!(
            "Usage: ./bananagrams [tiles]
Ex: ./bananagrams loremipsum -c -s -f common.txt
Options:
      -s to try shorter words first
      -l to try longer words first
      -c to check if valid at every step
      -r to randomize word choosing order
      -f to choose a file of words to draw from"
        );
        return;
    }
    let word_filename = after_flag_or("-f", "words.txt".to_string());
    // let word_file = File::open(word_filename).expect("no such file");
    let mut words: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(&word_filename) {
        for line in lines {
            if let Ok(word) = line {
                words.push(word);
            }
        }
    } else {
        println!("file '{}' not found", word_filename);
        return;
    }
    let tileword: String = getarg(1, "loremipsum".to_string());
    let tiles: Vec<char> = tileword.chars().collect();
    words = words
        .into_iter()
        .filter(|word| can_be_made_with(&word.as_str(), &tiles))
        .collect();
    if arg_exists("-r") {
        words.shuffle(&mut thread_rng());
    }
    if arg_exists("-s") {
        words.sort_by_key(|a| a.len());
    }
    if arg_exists("-l") {
        words.sort_by_key(|a| a.len());
        words.reverse();
    }
    println!("{:?}", words);

    let board_dim = &tiles.len() * 2;
    let mut board = Grid(Array2::from_elem((board_dim, board_dim), ' '));
    let mut minimum: Option<&Grid> = None;
    let mut minimum_area: usize = board_dim * board_dim;
    let mut wordstack: Vec<WordStackFrame> = Vec::new();
    wordstack.push(WordStackFrame {
        remaining_tiles: tiles,
        available_words: words,
        placed_letters: Vec::new(),
        recursion_depth: 0,
    });

    let mut hashed_boards: HashSet<u64> = HashSet::new();
}

//utils

//if the cmd line arg at index is parseable as a T, return that
//else return the default value
fn getarg<T: std::str::FromStr>(index: usize, default: T) -> T {
    match env::args().nth(index) {
        Some(arg) => arg.parse().unwrap_or(default),
        None => default,
    }
}

//if this arg was supplied, return its index
//else None
fn arg_pos(arg: &str) -> Option<usize> {
    for (i, argument) in env::args().enumerate() {
        if argument == arg {
            return Some(i);
        }
    }
    None
}

fn arg_exists(arg: &str) -> bool {
    arg_pos(arg).is_some()
}

fn after_flag_or<T: std::str::FromStr>(flag: &str, default: T) -> T {
    match arg_pos(flag) {
        Some(index) => getarg(index + 1, default),
        None => default,
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//structs
#[derive(Clone)]
struct LetterPlacement {
    letter: char,
    row: usize,
    col: usize,
}

#[derive(Clone)]
struct BoundingBox {
    min_col: usize,
    max_col: usize,
    min_row: usize,
    max_row: usize,
}

impl BoundingBox {
    fn area(&self) -> usize {
        max(
            (self.max_row - self.min_row + 1) * (self.max_col - self.min_col + 1),
            0,
        )
    }
}

#[derive(Clone)]
struct WordStackFrame {
    remaining_tiles: Vec<char>,
    available_words: Vec<String>,
    placed_letters: Vec<LetterPlacement>,
    recursion_depth: usize,
}

enum Direction {
    vertical,
    horizontal,
}

#[derive(Hash)]
struct Grid(Array2<char>);

impl Grid {
    fn print(&self) {
        println!("{}", self.0);
    }

    fn bounding_box(&self) -> BoundingBox {
        let width = self.0.dim().0;
        let height = self.0.dim().1;
        let mut min_col = width;
        let mut max_col = 0;
        let mut min_row = height;
        let mut max_row = 0;
        for r in 0..width {
            for c in 0..height {
                if (self.0[[r, c]] != ' ') {
                    min_col = min(min_col, c);
                    max_col = max(max_col, c);
                    min_row = min(min_row, r);
                    max_row = max(max_row, r);
                }
            }
        }
        BoundingBox {
            min_col,
            max_col,
            min_row,
            max_row,
        }
    }

    fn regex_for(&self, position: usize, dir: Direction, available_chars: &[char]) -> Regex {
        let words = self.words_at(position, dir);
        let chars_regex = format!("[{}]*", String::from_iter(available_chars));
        let regex_string = format!(
            "{}{}{}",
            &chars_regex,
            &words.trim().replace(' ', &chars_regex),
            &chars_regex
        );
        Regex::new(&regex_string).unwrap()
    }

    fn bounding_box_area(&self) -> usize {
        self.bounding_box().area()
    }

    fn valid_bananagrams(&self, words: &[String]) -> bool {
        let bounds = self.bounding_box();
        let mut words_to_check = Vec::new();
        for row in bounds.min_row..bounds.max_row + 1 {
            words_to_check.extend(self.words_at(row, horizontal).split());
        }
        for col in bounds.min_col..bounds.max_col + 1 {
            words_to_check.extend(self.words_at(col, vertical).split());
        }
        unimplemented!();
    }

    fn word_placements_for(
        &self,
        word: &str,
        position: usize,
        dir: Direction,
    ) -> Vec<Vec<LetterPlacement>> {
        unimplemented!();
    }

    fn words_at(&self, position: usize, dir: Direction) -> String {
        let chars = match dir {
            horizontal => self.0.slice(s![position..position + 1, ..]), //row
            vertical => self.0.slice(s![.., position..position + 1]),   //column
        };
        String::from_iter(chars)
    }

    fn insert(&mut self, r: usize, c: usize, val: char) {
        self.0[[r, c]] = val;
    }
}

fn can_be_made_with(word: &str, tiles: &[char]) -> bool {
    let mut tiles = tiles.to_owned();
    for c in word.chars() {
        match tiles.iter().position(|x| *x == c) {
            None => {
                return false;
            }
            Some(index) => {
                tiles.remove(index);
            }
        }
    }
    true
}

fn place_word_at(word: &str, c0: usize, r0: usize, dir: Direction) -> Vec<LetterPlacement> {
    let mut result = Vec::new();
    for (i, c) in word.chars().enumerate() {
        result.push(match &dir {
            horizontal => LetterPlacement {
                letter: c,
                col: c0 + i,
                row: r0,
            },
            vertical => LetterPlacement {
                letter: c,
                col: c0,
                row: r0 + i,
            },
        });
    }
    result
}

fn pop_stack() {
    unimplemented!();
}

fn find_minimum_area_configuration() {
    unimplemented!();
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn hash_grids() {
    let mut board = Grid(Array2::from_elem((5, 5), ' '));
    let empty_hash = hash(&board);
    board.insert(1, 1, 'h');
    assert!(hash(&board) != empty_hash);
    board.insert(1, 1, ' ');
    assert!(hash(&board) == empty_hash);
    board = Grid(Array2::from_elem((5, 5), ' '));
    assert!(hash(&board) == empty_hash);
}
