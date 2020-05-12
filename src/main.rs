use ndarray::Array2;
use rand::prelude::*;
use regex::Regex;
use std::{
    cell::RefCell,
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

macro_rules! time {
    ($description:literal,$code:block) => {
        println!("begin {}",$description);
        let time = std::time::Instant::now();
        $code
        println!("time elapsed: {:.5} seconds",time.elapsed().as_secs_f64());
        println!("end   {}",$description);
    };
}

thread_local! {
    static STATE: RefCell<SolveState> = RefCell::new(SolveState::default());
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

    let preemptive_checking = arg_exists("-c");
    let board_dim = tiles.len() * 2;
    STATE.with(|state| {
        state.replace(SolveState {
            board: Grid(Array2::from_elem((board_dim, board_dim), ' ')),
            minimum: None,
            minimum_area: board_dim * board_dim,
            wordstack: Vec::new(),
            hashed_boards: HashSet::new(),
        });
        state.borrow_mut().wordstack.push(WordStackFrame {
            remaining_tiles: tiles,
            available_words: words,
            placed_letters: Vec::new(),
            recursion_depth: 0,
        });
        find_minimum_area_configuration(preemptive_checking);
        if let Some(min) = &state.borrow_mut().minimum {
            println!("Minimum solution:");
            min.print();
        } else {
            print!("Impossible to solve with these tiles");
        }
    });
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
    remaining_tiles: Vec<char>,
    available_words: Vec<String>,
    placed_letters: Vec<LetterPlacement>,
    recursion_depth: usize,
}

enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Clone, Default)]
struct Grid(Array2<char>);

impl Hash for Grid {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let bounds = self.bounding_box();
        for r in bounds.min_row..bounds.max_row + 1 {
            for c in bounds.min_col..bounds.max_col + 1 {
                ::core::hash::Hash::hash(&self.0[[r, c]], state);
            }
        }
    }
}

impl Grid {
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

    fn place_letter(&mut self, pl: &LetterPlacement) {
        self.insert(pl.row, pl.col, pl.letter);
    }

    fn midpoint(&self) -> (usize, usize) {
        (self.0.dim().0 / 2, self.0.dim().1 / 2)
    }
}

#[derive(Default)]
struct SolveState {
    board: Grid,
    minimum: Option<Grid>,
    minimum_area: usize,
    wordstack: Vec<WordStackFrame>,
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

macro_rules! pop_stack {
    ($state:ident) => {
        let mut state = $state.borrow_mut();
        if let Some(stackframe) = state.wordstack.pop() {
            for placed_letter in stackframe.placed_letters {
                state
                    .board
                    .insert(placed_letter.row, placed_letter.col, ' ');
            }
        }
    };
}
fn find_minimum_area_configuration(preemptive_checking: bool) {
    STATE.with(|s| {
        let mystackframe = (*s.borrow_mut()).wordstack.last().unwrap().clone(); //TODO why do I need clone here?
        let mut tiles = mystackframe.remaining_tiles.clone();

        if mystackframe.recursion_depth > 0 {
            //actually place tiles we are assigned
            for ltr in &mystackframe.placed_letters {
                (*s.borrow_mut()).board.place_letter(&ltr);
                let index = tiles.iter().position(|x| *x == ltr.letter).unwrap();
                tiles.remove(index);
            }
        }

        let board = &(*s.borrow()).board.clone();

        //early exit checks
        let boardhash = hash_result(board);
        if (*s.borrow()).hashed_boards.contains(&boardhash) {
            pop_stack!(s);
            return;
        }
        (*s.borrow_mut()).hashed_boards.insert(boardhash);
        let area = board.bounding_box_area();
        if area > (*s.borrow()).minimum_area {
            pop_stack!(s);
            return;
        }
        if preemptive_checking && !board.valid_bananagrams(&mystackframe.available_words) {
            pop_stack!(s);
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
            pop_stack!(s);
            return;
        }

        //Base Case: we have an empty board and should place a first word
        if mystackframe.recursion_depth == 0 {
            for word in &mystackframe.available_words {
                println!("{}", &word);
                let midpoint = (*s.borrow()).board.midpoint();
                let placement = place_word_at(&word, midpoint.0, midpoint.1, Direction::Horizontal);
                (*s.borrow_mut()).wordstack.push(WordStackFrame {
                    remaining_tiles: mystackframe.remaining_tiles.clone(),
                    available_words: mystackframe.available_words.clone(),
                    placed_letters: placement,
                    recursion_depth: 1,
                });
                find_minimum_area_configuration(preemptive_checking);
            }
            pop_stack!(s);
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
                    //generate new stack frame
                    (*s.borrow_mut()).wordstack.push(WordStackFrame {
                        remaining_tiles: tiles.clone(),
                        available_words: mystackframe.available_words.clone(),
                        placed_letters: placement,
                        recursion_depth: &mystackframe.recursion_depth + 1,
                    });
                    //recurse
                    find_minimum_area_configuration(preemptive_checking);
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
                    //generate new stack frame
                    (*s.borrow_mut()).wordstack.push(WordStackFrame {
                        remaining_tiles: tiles.clone(),
                        available_words: mystackframe.available_words.clone(),
                        placed_letters: placement,
                        recursion_depth: &mystackframe.recursion_depth + 1,
                    });
                    //recurse
                    find_minimum_area_configuration(preemptive_checking);
                }
            }
        }
        pop_stack!(s);
    });
}

fn hash_result<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
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
    let empty_hash = hash_result(&board);
    board.insert(1, 1, 'h');
    assert!(hash_result(&board) != empty_hash);
    board.insert(1, 1, ' ');
    assert!(hash_result(&board) == empty_hash);
    let mut board2 = Grid(Array2::from_elem((5, 5), ' '));
    assert!(hash_result(&board2) == empty_hash);
}

#[test]
fn hash_offset() {
    let mut board = Grid(Array2::from_elem((5, 5), ' '));
    let empty_hash = hash_result(&board);
    board.insert(1, 1, 'h');
    board.insert(1, 2, 'i');
    board.insert(2, 1, 'i');
    let hi_hash = hash_result(&board);
    assert!(hash_result(&board) != empty_hash);
    board.insert(1, 1, ' ');
    board.insert(1, 2, ' ');
    board.insert(2, 1, ' ');
    assert!(hash_result(&board) == empty_hash);
    board.insert(2, 2, 'h');
    board.insert(2, 3, 'i');
    board.insert(3, 2, 'i');
    assert!(hash_result(&board) == hi_hash);
}
