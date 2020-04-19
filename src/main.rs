use rand::prelude::*;
use regex::Regex;
use std::env;
//try to derive Hash for board

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

fn main() {
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
