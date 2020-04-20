use ndarray::Array2;
use rand::prelude::*;
use regex::Regex;
use std::cmp::max;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

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
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct BoundingBox {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
}

impl BoundingBox {
    fn area(&self) -> usize {
        max((self.ymax - self.ymin + 1) * (self.xmax - self.xmin + 1), 0)
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
        unimplemented!();
    }

    fn bounding_box(&self) -> BoundingBox {
        unimplemented!();
    }

    fn bounding_box_area(&self) -> usize {
        self.bounding_box().area()
    }

    fn valid_bananagrams(&self) -> bool {
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

    fn insert(&mut self, x: usize, y: usize, val: char) {
        self.0[[x, y]] = val;
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

fn place_word_at(word: &str, x: usize, y: usize, dir: Direction) -> Vec<LetterPlacement> {
    let mut result = Vec::new();
    for (i, c) in word.chars().enumerate() {
        result.push(match &dir {
            horizontal => LetterPlacement {
                letter: c,
                x: x + i,
                y: y,
            },
            vertical => LetterPlacement {
                letter: c,
                x: x,
                y: y + i,
            },
        });
    }
    result
}

fn regex_for(position: usize, dir: Direction, available_chars: &[char]) -> Regex {
    unimplemented!();
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
