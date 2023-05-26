use std::io::{self, BufRead};

use regex::{Captures, Match, Regex};

fn main() {
    let stdin = io::stdin();
    let memory_regex = Regex::new(r"request completed using (.*)MB").unwrap();

    let mut ram_usage: Vec<f32> = Vec::new();

    let max_lines = 20;
    let mut line_count = 0;
    for line in stdin.lock().lines() {
        line_count += 1;
        let line = line.expect("Could not read line from standard in");

        let captures = memory_regex.captures(line.as_str());
        match captures {
            Some(v) => ram_usage.push(v.get(1).unwrap().as_str().parse().unwrap()),
            None => (),
        }

        if line_count >= max_lines {
            break;
        }
    }

    println!("{:?}", ram_usage);
}
