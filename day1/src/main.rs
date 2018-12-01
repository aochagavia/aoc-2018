use std::collections::HashSet;

const INPUT: &'static str = include_str!("input");

fn main() {
    let mut reached_freqs = HashSet::new();
    let mut freq = 0;

    reached_freqs.insert(freq);
    'a: loop {
        for line in INPUT.lines() {
            if line.len() < 2 {
                break;
            }

            let multiplier = if line.as_bytes()[0] == b'+' {
                1
            } else {
                -1
            };

            let number: i64 = line[1..].parse().unwrap();
            freq += number * multiplier;

            if !reached_freqs.insert(freq) {
                // The freq had already been seen
                println!("{}", freq);
                break 'a;
            }
        }
    }
    // println!("{}", reached_freqs.len());
    // println!("{}", freq);
}
