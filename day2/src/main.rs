use std::collections::HashMap;

const INPUT: &'static str = include_str!("input");

fn dist(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars()).map(|(c1, c2)| if c1 == c2 { 0 } else { 1 }).sum()
}

fn remove_distinct(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars()).filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None }).collect()
}

fn p2() -> String {
    let lines: Vec<_> = INPUT.lines().collect();
    for i in 0..lines.len() {
        let line = &lines[i];
        if line.len() == 0 {
            continue;
        }

        // Test against all remaining strings
        for other_line in &lines[i + 1..] {
            //println!("Comparing {} to {}", line, other_line);
            if dist(line, other_line) == 1 {
                // We have a match
                return remove_distinct(line, other_line);
            }
        }
    }

    unreachable!()
}

fn p1() -> i64 {
    let mut have_2_repeated = 0;
    let mut have_3_repeated = 0;
    for line in INPUT.lines() {
        if line.len() == 0 {
            continue;
        }

        // Get letter frequency
        let mut map = HashMap::new();
        for c in line.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        // Sort into categories
        let mut in_2 = false;
        let mut in_3 = false;
        for (v, freq) in map {
            if freq == 2 && !in_2 {
                have_2_repeated += 1;
                in_2 = true;
            } else if freq == 3 && !in_3 {
                have_3_repeated += 1;
                in_3 = true;
            }
        }
    }

    have_2_repeated * have_3_repeated
}

fn main() {
    let solution = p2();
    println!("{}", solution);
}
