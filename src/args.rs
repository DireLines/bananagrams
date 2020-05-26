use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

//if the cmd line arg at index is parseable as a T, return that
//else return the default value
pub fn getarg<T: std::str::FromStr>(index: usize, default: T) -> T {
    match env::args().nth(index) {
        Some(arg) => arg.parse().unwrap_or(default),
        None => default,
    }
}

//if this arg was supplied, return its index
//else None
pub fn arg_pos(arg: &str) -> Option<usize> {
    for (i, argument) in env::args().enumerate() {
        if argument == arg {
            return Some(i);
        }
    }
    None
}

pub fn arg_exists(arg: &str) -> bool {
    arg_pos(arg).is_some()
}

pub fn after_flag_or<T: std::str::FromStr>(flag: &str, default: T) -> T {
    match arg_pos(flag) {
        Some(index) => getarg(index + 1, default),
        None => default,
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn num_args() -> usize {
    env::args().count()
}
