use std::cmp;

const INPUT: &'static str = include_str!("input");

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn parse_claim(line: &str) -> Claim {
    let parts: Vec<_> = line.split_whitespace().collect();
    let id = parts[0][1..].parse().unwrap();

    // Part 1 is ignored

    let coords: Vec<_> = parts[2].split(',').collect();
    let x = coords[0].parse().unwrap();
    let y = coords[1][0..coords[1].len() - 1].parse().unwrap();

    let size: Vec<_> = parts[3].split('x').filter_map(|s| s.parse().ok()).collect();
    let width = size[0];
    let height = size[1];
    Claim {
        id,
        x,
        y,
        width,
        height
    }
}

#[test]
fn parse_claim_works() {
    let claim = parse_claim("#123 @ 42,5: 6x73");
    assert!(claim.id == 123);
    assert!(claim.x == 42);
    assert!(claim.y == 5);
    assert!(claim.width == 6);
    assert!(claim.height == 73);
}

fn p2() -> usize {
    // Width and height determined by scanning the input file
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;
    let mut map = vec![0; WIDTH * HEIGHT];

    for line in INPUT.lines().filter(|l| l.len() != 0) {
        let claim = parse_claim(line);
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                map[y * WIDTH + x] += 1;
            }
        }
    }

    for line in INPUT.lines().filter(|l| l.len() != 0) {
        let claim = parse_claim(line);
        let mut overlaps = false;
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                if map[y * WIDTH + x] > 1 {
                    overlaps = true;
                }
            }
        }

        if !overlaps {
            return claim.id;
        }
    }

    unreachable!()
}

fn p1() -> usize {
    // Width and height determined by scanning the input file
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;
    let mut map = vec![0; WIDTH * HEIGHT];

    for line in INPUT.lines().filter(|l| l.len() != 0) {
        let claim = parse_claim(line);
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                map[y * WIDTH + x] += 1;
            }
        }
    }

    map.into_iter().filter(|&x| x > 1).count()
}

fn main() {
    let solution = p2();
    println!("{}", solution);
}
