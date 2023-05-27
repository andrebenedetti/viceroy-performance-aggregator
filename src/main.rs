use core::fmt;
use std::io::{self, BufRead};

use regex::Regex;

struct Measurements {
    avg: i32,
    p95: i32,
    p99: i32,
}

fn aggregate_sorted_measurements(measurements: &Vec<i32>) -> Measurements {
    let mut sum = 0;
    let len = measurements.len();
    for m in measurements {
        sum += m;
    }

    Measurements {
        avg: sum / len as i32,
        p95: measurements[(0.95 * measurements.len() as f32) as usize],
        p99: measurements[(0.99 * measurements.len() as f32) as usize],
    }
}

impl fmt::Display for Measurements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "avg: {}\np95: {}\np99: {}", self.avg, self.p95, self.p99)
    }
}

fn main() {
    let stdin = io::stdin();
    let memory_regex = Regex::new(r"request completed using (.*) MB").unwrap();
    let run_time_regex = Regex::new(r"request completed in (.*)(ms|µs)").unwrap();

    let mut ram_usage: Vec<i32> = Vec::new();
    let mut run_time_us: Vec<i32> = Vec::new();

    let max_lines = 200;
    let mut line_count = 0;
    for line in stdin.lock().lines() {
        line_count += 1;
        let line = line.expect("Could not read line from standard in");

        if let Some(captures) = memory_regex.captures(line.as_str()) {
            if let Some(m) = captures.get(1) {
                match m.as_str().parse::<i32>() {
                    Ok(v) => {
                        ram_usage.push(v);
                    }
                    Err(_) => (),
                }
            }
        }

        let captures = run_time_regex.captures(line.as_str());
        match captures {
            Some(v) => {
                let time_unit = v.get(2).unwrap().as_str();
                let run_time = v.get(1).unwrap().as_str().parse().unwrap();

                if time_unit == "ms" {
                    run_time_us.push(run_time * 1000);
                } else {
                    run_time_us.push(run_time);
                }
            }
            None => (),
        }

        if line_count >= max_lines {
            break;
        }
    }

    run_time_us.sort_unstable();
    println!("{}", aggregate_sorted_measurements(&run_time_us));
}
