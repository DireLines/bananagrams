use lazy_static::lazy_static;
use ndarray::s;
use ndarray::Array2;
use rand::prelude::*;
use regex::Regex;
use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::{hash_map::DefaultHasher, HashSet},
    hash::Hasher,
    iter::FromIterator,
};

mod args;
use args::*;

//mutable static
thread_local! {
    static STATE: RefCell<SolveState> = RefCell::new(SolveState::default());
}

//immutable static
lazy_static! {
    static ref PREEMPTIVE_CHECKING: bool = arg_exists("-c");
}

fn main() {
    if num_args() < 2 || arg_exists("-help") {
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

    let board_dim = tiles.len() * 2;
    STATE.with(|state| {
        state.replace(SolveState {
            minimum: None,
            minimum_area: board_dim * board_dim,
            hashed_boards: HashSet::new(),
        });
        find_minimum_area_configuration(WordStackFrame {
            board: Grid(Array2::from_elem((board_dim, board_dim), ' ')),
            remaining_tiles: tiles,
            available_words: words,
            placed_letters: Vec::new(),
            recursion_depth: 0,
        });
        if let Some(min) = &state.borrow_mut().minimum {
            println!("Minimum solution:");
            min.print();
        } else {
            print!("Impossible to solve with these tiles");
        }
    });
}

//structs
#[derive(Debug, Clone)]
struct LetterPlacement {
    letter: char,
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct WordStackFrame {
    board: Grid,
    remaining_tiles: Vec<char>,
    available_words: Vec<String>,
    placed_letters: Vec<LetterPlacement>,
    recursion_depth: usize,
}

enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Default)]
struct Grid(Array2<char>);

impl Grid {
    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let bounds = self.bounding_box();
        for r in bounds.min_row..bounds.max_row + 1 {
            for c in bounds.min_col..bounds.max_col + 1 {
                ::core::hash::Hash::hash(&self.0[[r, c]], &mut hasher);
            }
        }
        let self_hash = hasher.finish();
        hasher = DefaultHasher::new();
        for c in bounds.min_col..bounds.max_col + 1 {
            for r in bounds.min_row..bounds.max_row + 1 {
                ::core::hash::Hash::hash(&self.0[[r, c]], &mut hasher);
            }
        }
        let transpose_hash = hasher.finish();
        self_hash | transpose_hash
    }
    fn print(&self) {
        for row in 0..self.0.dim().0 {
            for col in 0..self.0.dim().1 {
                print!("{} ", self.0[[row, col]]);
            }
            print!("\n");
        }
        println!("Area: {}", self.bounding_box_area());
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
                if self.0[[r, c]] != ' ' {
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
            "^{}{}{}$",
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
        let mut words_to_check: Vec<String> = Vec::new();
        for row in bounds.min_row..bounds.max_row + 1 {
            words_to_check.extend(
                self.words_at(row, Direction::Horizontal)
                    .split_whitespace()
                    .map(|x| x.to_string()),
            );
        }
        for col in bounds.min_col..bounds.max_col + 1 {
            words_to_check.extend(
                self.words_at(col, Direction::Vertical)
                    .split_whitespace()
                    .map(|x| x.to_string()),
            );
        }
        for word in &words_to_check {
            if !words.contains(word) && word.len() > 1 {
                return false;
            }
        }
        true
    }

    fn word_placements_for(
        &self,
        word: &str,
        position: usize,
        dir: Direction,
    ) -> Vec<Vec<LetterPlacement>> {
        let mut result = Vec::new();
        let bounds = self.bounding_box();
        let lower = match dir {
            Direction::Horizontal => bounds.min_col,
            Direction::Vertical => bounds.min_row,
        };
        for i in lower - word.len()..lower + 1 {
            let mut this_result: Vec<LetterPlacement> = Vec::new();
            let mut connected: bool = false;
            for j in 0..word.len() {
                let row = match dir {
                    Direction::Horizontal => position,
                    Direction::Vertical => i + j,
                };
                let col = match dir {
                    Direction::Horizontal => i + j,
                    Direction::Vertical => position,
                };
                let letter = word.chars().nth(j).unwrap();
                match self.get(row, col) {
                    ' ' => {
                        this_result.push(LetterPlacement { letter, row, col });
                        continue;
                    }
                    x if x == letter => connected = true,
                    _ => break,
                };
            }
            if !this_result.is_empty() && connected {
                result.push(this_result);
            }
        }
        result
    }

    fn words_at(&self, position: usize, dir: Direction) -> String {
        let chars = match dir {
            Direction::Horizontal => self.0.slice(s![position..position + 1, ..]), //row
            Direction::Vertical => self.0.slice(s![.., position..position + 1]),   //column
        };
        String::from_iter(chars)
    }

    fn insert(&mut self, r: usize, c: usize, val: char) {
        self.0[[r, c]] = val;
    }

    fn get(&self, r: usize, c: usize) -> char {
        self.0[[r, c]]
    }

    fn clear(&mut self) {
        for row in 0..self.0.dim().0 {
            for col in 0..self.0.dim().1 {
                self.insert(row, col, ' ');
            }
        }
    }

    fn place_letter(&mut self, pl: &LetterPlacement) {
        self.insert(pl.row, pl.col, pl.letter);
    }

    fn midpoint(&self) -> (usize, usize) {
        (self.0.dim().0 / 2, self.0.dim().1 / 2)
    }
}

#[derive(Default)]
struct SolveState {
    minimum: Option<Grid>,
    minimum_area: usize,
    hashed_boards: HashSet<u64>,
}

fn can_be_made_with(word: &str, tiles: &[char]) -> bool {
    let mut tiles = tiles.to_owned();
    for c in word.chars() {
        match tiles.iter().position(|x| *x == c) {
            None => return false,
            Some(index) => tiles.remove(index),
        };
    }
    true
}

fn place_word_at(word: &str, c0: usize, r0: usize, dir: Direction) -> Vec<LetterPlacement> {
    let mut result = Vec::new();
    for (i, c) in word.chars().enumerate() {
        result.push(match dir {
            Direction::Horizontal => LetterPlacement {
                letter: c,
                col: c0 + i,
                row: r0,
            },
            Direction::Vertical => LetterPlacement {
                letter: c,
                col: c0,
                row: r0 + i,
            },
        });
    }
    result
}
fn find_minimum_area_configuration(mystackframe: WordStackFrame) {
    STATE.with(|s| {
        let mut tiles = mystackframe.remaining_tiles.clone();
        let mut board = mystackframe.board;
        if mystackframe.recursion_depth > 0 {
            //actually place tiles we are assigned
            for ltr in &mystackframe.placed_letters {
                board.place_letter(&ltr);
                let index = tiles.iter().position(|x| *x == ltr.letter).unwrap();
                tiles.remove(index);
            }
        }

        //will not modify board from now on
        let board = board;

        //early exit checks
        let boardhash = board.hash();
        if (*s.borrow()).hashed_boards.contains(&boardhash) {
            return;
        }
        (*s.borrow_mut()).hashed_boards.insert(boardhash);
        let area = board.bounding_box_area();
        if area > (*s.borrow()).minimum_area {
            return;
        }
        if *PREEMPTIVE_CHECKING && !board.valid_bananagrams(&mystackframe.available_words) {
            return;
        }

        //Base Case: we are out of tiles so we found a solution
        if tiles.is_empty() {
            if board.valid_bananagrams(&mystackframe.available_words)
                && ((*s.borrow()).minimum.is_none() || area < (*s.borrow()).minimum_area)
            {
                (*s.borrow_mut()).minimum = Some(board.clone());
                (*s.borrow_mut()).minimum_area = area;
                println!("New Smallest Solution Found!");
                board.print();
            }
            return;
        }

        //Base Case: we have an empty board and should place a first word
        if mystackframe.recursion_depth == 0 {
            for word in &mystackframe.available_words {
                println!("{}", &word);
                let midpoint = board.midpoint();
                let placement = place_word_at(&word, midpoint.0, midpoint.1, Direction::Horizontal);
                find_minimum_area_configuration(WordStackFrame {
                    board: board.clone(),
                    remaining_tiles: mystackframe.remaining_tiles.clone(),
                    available_words: mystackframe.available_words.clone(),
                    placed_letters: placement,
                    recursion_depth: 1,
                });
            }
            return;
        }

        let bounds = board.bounding_box();
        for row in bounds.min_row..bounds.max_row + 1 {
            let regex = board.regex_for(row, Direction::Horizontal, &mystackframe.remaining_tiles);
            let newwords: Vec<String> = mystackframe
                .available_words
                .iter()
                .filter(|w| regex.is_match(w))
                .map(|w| w.to_string())
                .collect();
            for word in &newwords {
                let word_placements = board.word_placements_for(&word, row, Direction::Horizontal);
                for placement in word_placements {
                    //check if word can be made
                    let tilesplaced: String = placement.iter().map(|lp| lp.letter).collect();
                    if !can_be_made_with(&tilesplaced, &tiles) {
                        continue;
                    }
                    //recurse
                    find_minimum_area_configuration(WordStackFrame {
                        board: board.clone(),
                        remaining_tiles: tiles.clone(),
                        available_words: mystackframe.available_words.clone(),
                        placed_letters: placement,
                        recursion_depth: &mystackframe.recursion_depth + 1,
                    });
                }
            }
        }
        for col in bounds.min_col..bounds.max_col + 1 {
            let regex = board.regex_for(col, Direction::Vertical, &mystackframe.remaining_tiles);
            let newwords: Vec<String> = mystackframe
                .available_words
                .iter()
                .filter(|w| regex.is_match(w))
                .map(|w| w.to_string())
                .collect();
            for word in &newwords {
                let word_placements = board.word_placements_for(&word, col, Direction::Vertical);
                for placement in word_placements {
                    //check if word can be made
                    let tilesplaced: String = placement.iter().map(|lp| lp.letter).collect();
                    if !can_be_made_with(&tilesplaced, &tiles) {
                        continue;
                    }
                    //recurse
                    find_minimum_area_configuration(WordStackFrame {
                        board: board.clone(),
                        remaining_tiles: tiles.clone(),
                        available_words: mystackframe.available_words.clone(),
                        placed_letters: placement,
                        recursion_depth: &mystackframe.recursion_depth + 1,
                    });
                }
            }
        }
    });
}

#[test]
fn bounding_box() {
    let mut grid = Grid(Array2::from_elem((10, 10), ' '));
    grid.insert(5, 5, 'o');
    let bounds = grid.bounding_box();
    assert_eq!(bounds.min_row, 5);
    assert_eq!(bounds.max_row, 5);
    assert_eq!(bounds.min_col, 5);
    assert_eq!(bounds.max_col, 5);
    grid.insert(7, 6, 'o');
    let bounds = grid.bounding_box();
    assert_eq!(bounds.min_row, 5);
    assert_eq!(bounds.max_row, 7);
    assert_eq!(bounds.min_col, 5);
    assert_eq!(bounds.max_col, 6);
    grid.insert(3, 4, 'o');
    let bounds = grid.bounding_box();
    assert_eq!(bounds.min_row, 3);
    assert_eq!(bounds.max_row, 7);
    assert_eq!(bounds.min_col, 4);
    assert_eq!(bounds.max_col, 6);
    assert_eq!(grid.bounding_box_area(), 15);
}

#[test]
fn regex() {
    let regex_string = "^a[ab]*a$";
    let r = Regex::new(&regex_string).unwrap();
    assert!(r.is_match("aa"));
    assert!(r.is_match("aba"));
    assert!(r.is_match("abbabababababbaa"));
    assert!(!r.is_match("a"));
    assert!(!r.is_match("abaca"));
    assert!(!r.is_match("cabac"));
}

#[test]
fn hash_grids() {
    let mut board = Grid(Array2::from_elem((5, 5), ' '));
    let empty_hash = board.hash();
    board.insert(1, 1, 'h');
    assert!(board.hash() != empty_hash);
    board.insert(1, 1, ' ');
    assert!(board.hash() == empty_hash);
    let mut board2 = Grid(Array2::from_elem((5, 5), ' '));
    assert!(board2.hash() == empty_hash);
}

#[test]
fn hash_offset() {
    let mut board = Grid(Array2::from_elem((5, 5), ' '));
    let empty_hash = board.hash();
    board.insert(1, 1, 'h');
    board.insert(1, 2, 'i');
    board.insert(2, 1, 'i');
    let hi_hash = board.hash();
    assert!(board.hash() != empty_hash);
    board.clear();
    assert!(board.hash() == empty_hash);
    board.insert(2, 2, 'h');
    board.insert(2, 3, 'i');
    board.insert(3, 2, 'i');
    assert!(board.hash() == hi_hash);
}

#[test]
fn hash_transpose() {
    let mut board = Grid(Array2::from_elem((5, 5), ' '));
    let empty_hash = board.hash();
    board.insert(1, 1, 'h');
    board.insert(1, 2, 'i');
    board.insert(2, 1, 'e');
    board.insert(3, 1, 'y');
    let hi_hey_hash = board.hash();
    assert!(board.hash() != empty_hash);
    board.clear();
    assert!(board.hash() == empty_hash);
    board.insert(1, 1, 'h');
    board.insert(1, 2, 'e');
    board.insert(1, 3, 'y');
    board.insert(2, 1, 'i');
    assert!(board.hash() == hi_hey_hash);
    board.insert(1, 1, ' ');
    assert!(board.hash() != hi_hey_hash);
}
