use std::cmp;
use std::{u32, i32, usize};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dist {
    Unknown,
    Owned(usize, u32)
}

const SHARED_OWNER_ID: usize = usize::MAX;

fn p1() -> usize {
    let mut min_x = u32::MAX;
    let mut max_x = 0;
    let mut min_y = u32::MAX;
    let mut max_y = 0;

    // Parse input and get edges of the bounding box
    let mut all_coords = Vec::new();
    for line in INPUT.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut coords = line.split(',').map(|s| s.trim().parse().unwrap());
        let x: u32 = coords.next().unwrap();
        let y: u32 = coords.next().unwrap();

        all_coords.push((x, y));

        min_x = cmp::min(min_x, x);
        max_x = cmp::max(max_x, x);
        min_y = cmp::min(min_y, y);
        max_y = cmp::max(max_y, y);
    }

    // Create a 2d map where each cell stores its owner and the distance from the starting point
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut map: Vec<_> = vec![Dist::Unknown; (width * height) as usize];
    for (i, &(x, y)) in all_coords.iter().enumerate() {
        // Note: we need to translate the coords to be (0, 0) based
        let trans_x = (x - min_x) as usize;
        let trans_y = (y - min_y) as usize;

        // Set the starting points for each coordinate
        map[trans_y * width as usize + trans_x] = Dist::Owned(i, 0);
    }

    // Calculate the shortest path to each cell in the map
    let mut changed = true;
    while changed {
        changed = false;
        // For all cells that have an owner, see if we can get a faster route than current ones
        for i in 0..map.len(){
            if let Dist::Owned(owner_id, dist) = map[i] {
                changed = update_neighbors(&mut map, i, width, height, owner_id, dist + 1) || changed;
            }
        }
    }

    // The line below should print the map like the example shown on the AoC website
    // With points for equally distant cells and letters for cells that are reachable from
    // a given starting point
    // print_map(&map, width);

    // Calculate the sizes of all the regions
    let mut sizes = vec![0; all_coords.len()];
    for &cell in &map {
        match cell {
            Dist::Owned(SHARED_OWNER_ID, _) => (),
            Dist::Owned(owner_id, _) => sizes[owner_id] += 1,
            _ => ()
        }
    }

    // Identify infinite regions (all those that have cells at the border)
    let mut infinite = vec![false; all_coords.len()];
    for x in 0..width as usize {
        for &y in &[0, height as usize - 1] {
            let cell = map[y * width as usize + x];
            match cell {
                Dist::Owned(SHARED_OWNER_ID, _) => (),
                Dist::Owned(owner_id, _) => infinite[owner_id] = true,
                _ => ()
            }
        }
    }
    for y in 0..height as usize {
        for &x in &[0, width as usize - 1] {
            let cell = map[y * width as usize + x];
            match cell {
                Dist::Owned(SHARED_OWNER_ID, _) => (),
                Dist::Owned(owner_id, _) => infinite[owner_id] = true,
                _ => ()
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

fn update_neighbors(map: &mut [Dist], i: usize, width: u32, height: u32, new_owner_id: usize, new_dist: u32) -> bool {
    let mut changed = false;
    for neighbor in neighbors(i, width as usize, height as usize) {
        match map[neighbor] {
            Dist::Unknown => {
                // Unknown cells are always claimed
                map[neighbor] = Dist::Owned(new_owner_id, new_dist);
                changed = true;
            }
            Dist::Owned(owner_id, current_dist) => {
                if new_dist == current_dist && new_owner_id != owner_id {
                    // Owned cells become shared ones if there is more than one route with similar dist
                    let new_owner = Dist::Owned(SHARED_OWNER_ID, new_dist);
                    if map[neighbor] != new_owner {
                        map[neighbor] = new_owner;
                        changed = true;
                    }
                } else if new_dist < current_dist {
                    // We have found a faster route, so the owner *and* the dist need to be updated
                    map[neighbor] = Dist::Owned(new_owner_id, new_dist);
                    changed = true;
                }
            }
        }
    }

    changed
}

fn neighbors(i: usize, width: usize, height: usize) -> impl Iterator<Item=usize> {
    let i = i as i64;
    let width = width as i64;
    let height = height as i64;

    // Transform i to (x, y) coords
    let y = i / width;
    let x = i % width;

    // println!("{}, {}", x, y);

    [(-1, 0), (1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(move |(x_offset, y_offset)| (x_offset + x, y_offset + y))
        .filter(move |&(neighbor_x, neighbor_y)| {
            let within_bounds = 0 <= neighbor_x && neighbor_x < width && 0 <= neighbor_y && neighbor_y < height;

            // println!("Filtering: {}, {} (keep = {})", neighbor_x, neighbor_y, keep);
            within_bounds
        })
        .map(move |(neighbor_x, neighbor_y)| (neighbor_y * width + neighbor_x) as usize)
}

fn main() {
    let solution = p1();
    println!("{}", solution);
}

fn print_map(map: &[Dist], width: u32) {
    // Print map
    for line in map.chunks(width as usize) {
        for cell in line {
            let c = match cell {
                Dist::Owned(SHARED_OWNER_ID, _) => '.',
                Dist::Owned(owner_id, 0) => (*owner_id as u8 + b'A') as char,
                Dist::Owned(owner_id, _) => (*owner_id as u8 + b'a') as char,
                Dist::Unknown => '?'
            };
            print!("{}", c);
        }
        println!();
    }

    println!("\n---------\n");
}
