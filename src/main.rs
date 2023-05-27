use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let memory_regex = Regex::new(r"request completed using (.*) MB").unwrap();
    let run_time_regex = Regex::new(r"request completed in (.*)(ms|µs)").unwrap();

    let mut ram_usage: Vec<f32> = Vec::new();
    let mut run_time_us: Vec<f32> = Vec::new();

    let max_lines = 200;
    let mut line_count = 0;
    for line in stdin.lock().lines() {
        line_count += 1;
        let line = line.expect("Could not read line from standard in");

        let captures = memory_regex.captures(line.as_str());

        match captures {
            Some(v) => {
                match v.get(1) {
                    Some(m) => match m.as_str().parse::<f32>() {
                        Ok(v) => {
                            ram_usage.push(v);
                        }
                        Err(_) => (),
                    },
                    None => (),
                };
            }
            None => (),
        }
        let captures = run_time_regex.captures(line.as_str());
        match captures {
            Some(v) => {
                let time_unit = v.get(2).unwrap().as_str();
                let run_time = v.get(1).unwrap().as_str().parse().unwrap();

                if time_unit == "ms" {
                    run_time_us.push(run_time * 1000.0);
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

    println!("{:?}", ram_usage);
    println!("{:?}", run_time_us);
}
