use std::io;

use regex::Regex;
use std::io::BufRead;

fn parse_line(regex: &Regex, text: &str) -> Option<i32> {
    if let Some(captures) = regex.captures(text) {
        if let Some(m) = captures.get(1) {
            match m.as_str().parse::<i32>() {
                Ok(v) => return Some(v),
                Err(_) => return None,
            }
        }
    }

    None
}

pub fn capture_measurements() -> (Vec<i32>, Vec<i32>) {
    let stdin = io::stdin();
    let memory_regex = Regex::new(r"request completed using (.*) MB").unwrap();
    let run_time_regex_us = Regex::new(r"request completed in (.*)µs").unwrap();
    let run_time_regex_ms = Regex::new(r"request completed in (.*)ms").unwrap();

    let mut ram_usage: Vec<i32> = Vec::new();
    let mut run_time_us: Vec<i32> = Vec::new();

    let max_lines = 200;
    let mut line_count = 0;
    for line in stdin.lock().lines() {
        line_count += 1;
        let line = line.expect("Could not read line from standard in");

        if let Some(ram_datapoint) = parse_line(&memory_regex, line.as_str()) {
            ram_usage.push(ram_datapoint);
        }

        if let Some(run_time_ms_datapoint) = parse_line(&run_time_regex_ms, line.as_str()) {
            run_time_us.push(run_time_ms_datapoint * 1000);
        }

        if let Some(run_time_us_datapoint) = parse_line(&run_time_regex_us, line.as_str()) {
            run_time_us.push(run_time_us_datapoint);
        }

        if line_count >= max_lines {
            break;
        }
    }

    return (ram_usage, run_time_us);
}
