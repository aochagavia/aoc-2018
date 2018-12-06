use std::{cmp, i32, usize};
use std::collections::HashMap;

const INPUT: &'static str = include_str!("input");

fn p2() -> usize {
    let mut min_x = i32::MAX;
    let mut max_x = 0;
    let mut min_y = i32::MAX;
    let mut max_y = 0;

    // Parse input and get edges of the bounding box
    let mut all_coords = Vec::new();
    for line in INPUT.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut coords = line.split(',').map(|s| s.trim().parse().unwrap());
        let x: i32 = coords.next().unwrap();
        let y: i32 = coords.next().unwrap();

        all_coords.push((x, y));

        min_x = cmp::min(min_x, x);
        max_x = cmp::max(max_x, x);
        min_y = cmp::min(min_y, y);
        max_y = cmp::max(max_y, y);
    }

    let mut count = 0;
    let threshold = 10000;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut dist = 0;
            for (source_x, source_y) in &all_coords {
                dist += (x - source_x).abs() + (y - source_y).abs();
            }
            if dist < threshold {
                count += 1;
            }
        }
    }

    count
}

const SHARED_OWNER_ID: usize = usize::MAX;

fn p1() -> usize {
    let mut min_x = i32::MAX;
    let mut max_x = 0;
    let mut min_y = i32::MAX;
    let mut max_y = 0;

    // Parse input and get edges of the bounding box
    let mut all_coords = Vec::new();
    for line in INPUT.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut coords = line.split(',').map(|s| s.trim().parse().unwrap());
        let x: i32 = coords.next().unwrap();
        let y: i32 = coords.next().unwrap();

        all_coords.push((x, y));

        min_x = cmp::min(min_x, x);
        max_x = cmp::max(max_x, x);
        min_y = cmp::min(min_y, y);
        max_y = cmp::max(max_y, y);
    }

    // Create a map with from coordinates to their owner and the distance from it
    //
    // Note: this algorithm performs terribly, but works well enough since our input is small.
    // For bigger inputs you would probably use something graph-based
    let mut map = HashMap::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut owner_id = 0;
            let mut min_dist = i32::MAX;
            for (id, &(source_x, source_y)) in all_coords.iter().enumerate() {
                let dist = (x - source_x).abs() + (y - source_y).abs();
                if dist == min_dist {
                    owner_id = SHARED_OWNER_ID;
                } else if dist < min_dist {
                    owner_id = id;
                    min_dist = dist;
                }
            }

            map.insert((x, y), (owner_id, min_dist as u32));
        }
    }

    // Calculate the sizes of all the regions
    let mut sizes = vec![0; all_coords.len()];
    for (_, &(owner_id, _)) in &map {
        if owner_id != SHARED_OWNER_ID {
            sizes[owner_id] += 1;
        }
    }

    // Identify infinite regions (all those that have cells at the border)
    let mut infinite = vec![false; all_coords.len()];
    for x in min_x..=max_x {
        for &y in &[min_y, max_y] {
            let (owner_id, _) = map[&(x, y)];
            if owner_id != SHARED_OWNER_ID {
                infinite[owner_id] = true;
            }
        }
    }
    for y in min_y..=max_y {
        for &x in &[min_x, max_x] {
            let (owner_id, _) = map[&(x, y)];
            if owner_id != SHARED_OWNER_ID {
                infinite[owner_id] = true;
            }
        }
    }

    // Get the size of the biggest region, ignoring infinite ones
    sizes
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| !infinite[i])
        .map(|(_, size)| size)
        .max()
        .unwrap()
}

fn main() {
    let solution = p1();
    println!("{}", solution);
}

fn print_map(map: &HashMap<(i32, i32), (usize, u32)>, xs: (impl Iterator<Item=i32> + Clone), ys: impl Iterator<Item=i32>) {
    for y in ys {
        for x in xs.clone() {
            let c = match map[&(x, y)] {
                (SHARED_OWNER_ID, _) => '.',
                (owner_id, 0) => (owner_id as u8 + b'A') as char,
                (owner_id, _) => (owner_id as u8 + b'a') as char,
            };

            print!("{}", c);
        }

        println!();
    }
}
