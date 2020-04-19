use std::env;

use regex::Regex;

// import hashlib
//try to derive Hash for board

use rand::prelude::*;

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
}

fn main() {
    println!("Hello, world!");
}

fn can_be_made_with(word: &str, tiles: &Vec<char>) -> bool {
    unimplemented!();
}

fn place_word_at(word: &str, x: usize, y: usize, dir: Direction) {
    unimplemented!();
}

fn words_at(position: usize, dir: Direction) -> String {
    unimplemented!();
}

fn check_valid_bananagrams(grid: Grid) -> bool {
    unimplemented!();
}

fn regex_for(position: usize, dir: Direction, available_chars: &Vec<char>) -> Regex {
    unimplemented!();
}

fn word_placements_for(
    word: &str,
    position: usize,
    dir: Direction,
    grid: Grid,
) -> Vec<LetterPlacement> {
    unimplemented!();
}

fn pop_stack() {
    unimplemented!();
}

fn find_minimum_area_configuration() {
    unimplemented!();
}
