use rand::prelude::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

//macros
macro_rules! typecheck {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        let _: () = $e;
    };
}

fn main() {
    let word_filename = flag_or("-f", "words.txt".to_string());
    let word_file = File::open(word_filename).expect("no such file");
    println!(
        "Usage: ./bananagrams [tiles]
Ex: ./bananagrams loremipsum -c -s -f common.txt
Options:
      -s to try shorter words first
      -l to try longer words first
      -c to check if valid at every step
      -r to randomize alphabetical order
      -f to choose a file of words to draw from"
    );
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

fn flag_or<T: std::str::FromStr>(flag: &str, default: T) -> T {
    match arg_pos(flag) {
        Some(index) => getarg(index + 1, default),
        None => default,
    }
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
    fn area() -> usize {
        unimplemented!();
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
    Vertical,
    Horizontal,
}

#[derive(Hash)]
struct Grid(ndarray::Array2<char>);

impl Grid {
    fn print(&self) {
        unimplemented!();
    }

    fn bounding_box(&self) -> BoundingBox {
        unimplemented!();
    }

    fn bounding_box_area(&self) -> usize {
        unimplemented!();
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
        unimplemented!();
    }
}

fn can_be_made_with(word: &str, tiles: &Vec<char>) -> bool {
    unimplemented!();
}

fn place_word_at(word: &str, x: usize, y: usize, dir: Direction) -> Vec<LetterPlacement> {
    unimplemented!();
}

fn regex_for(position: usize, dir: Direction, available_chars: &Vec<char>) -> Regex {
    unimplemented!();
}

fn pop_stack() {
    unimplemented!();
}

fn find_minimum_area_configuration() {
    unimplemented!();
}
