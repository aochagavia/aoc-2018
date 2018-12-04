use std::collections::HashMap;

const INPUT: &'static str = include_str!("input");

#[derive(Debug, PartialEq, Eq)]
struct Log {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    event: Event,
}

#[derive(Debug, PartialEq, Eq)]
enum Event {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

#[test]
fn test_parse_log() {
    let log = "[1518-11-05 00:55] wakes up";
    assert!(parse_log(log) == Log {
        month: 11,
        day: 5,
        hour: 0,
        minute: 55,
        event: Event::WakeUp,
    });

    let log = "[1518-11-05 00:55] falls asleep";
    assert!(parse_log(log) == Log {
        month: 11,
        day: 5,
        hour: 0,
        minute: 55,
        event: Event::FallAsleep,
    });

    let log = "[1518-11-05 00:55] Guard #99 begins shift";
    assert!(parse_log(log) == Log {
        month: 11,
        day: 5,
        hour: 0,
        minute: 55,
        event: Event::BeginShift(99),
    });
}

fn parse_log(s: &str) -> Log {
    let month = s[6..8].parse().unwrap();
    let day = s[9..11].parse().unwrap();
    let hour = s[12..14].parse().unwrap();
    let minute = s[15..17].parse().unwrap();
    let event_str = &s[19..];

    let event = match event_str {
        "wakes up" => Event::WakeUp,
        "falls asleep" => Event::FallAsleep,
        _ => {
            let id = event_str.split_whitespace().nth(1).map(|id_str| id_str[1..].parse().unwrap()).unwrap();
            Event::BeginShift(id)
        }
    };

    Log {
        month,
        day,
        hour,
        minute,
        event,
    }
}

fn p2() -> u32 {
    let mut guard_map: HashMap<_, Vec<u32>> = HashMap::new();

    let mut current_guard = 0;
    let mut fall_asleep_minute = 0;

    let mut lines: Vec<_> = INPUT.lines().collect();
    lines.sort_unstable();
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let log = parse_log(line);

        match log.event {
            Event::FallAsleep => {
                fall_asleep_minute = log.minute;
            }
            Event::WakeUp => {
                // Track sleeping time for each minute
                let guard_log = guard_map.get_mut(&current_guard).unwrap();
                for minute in fall_asleep_minute..log.minute {
                    guard_log[minute as usize] += 1;
                }
            }
            Event::BeginShift(id) => {
                current_guard = id;
                fall_asleep_minute = 0;
                guard_map.entry(id).or_insert_with(|| vec![0; 60]);
            }
        }
    }

    let (guard_id, minute, _freq) = guard_map
        .iter()
        .map(|(&id, log)| {
            let max_minute = max_minute(&log);
            (id, max_minute, log[max_minute as usize])
        })
        .max_by_key(|&(_, _, freq)| freq)
        .unwrap();

    println!("Chosen guard: {}", guard_id);
    println!("Minute: {}", minute);

    guard_id * minute
}

fn max_minute(log: &[u32]) -> u32 {
    log.iter().enumerate().max_by_key(|(_, &freq)| freq).unwrap().0 as u32
}

fn p1() -> u32 {
    let mut guard_map: HashMap<_, Vec<u32>> = HashMap::new();

    let mut current_guard = 0;
    let mut fall_asleep_minute = 0;

    let mut lines: Vec<_> = INPUT.lines().collect();
    lines.sort_unstable();
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let log = parse_log(line);

        match log.event {
            Event::FallAsleep => {
                fall_asleep_minute = log.minute;
            }
            Event::WakeUp => {
                // Track sleeping time for each minute
                let guard_log = guard_map.get_mut(&current_guard).unwrap();
                for minute in fall_asleep_minute..log.minute {
                    guard_log[minute as usize] += 1;
                }
            }
            Event::BeginShift(id) => {
                current_guard = id;
                fall_asleep_minute = 0;
                guard_map.entry(id).or_insert_with(|| vec![0; 60]);
            }
        }
    }

    let (guard_id, guard_log) = guard_map.iter().max_by(|(_, log1), (_, log2)| log1.iter().sum::<u32>().cmp(&log2.iter().sum::<u32>())).unwrap();
    let minute = guard_log.iter().enumerate().max_by(|(_, time1), (_, time2)| time1.cmp(time2)).unwrap().0;
    println!("Guard id: {}", guard_id);
    println!("Minute: {}", minute);
    guard_id * minute as u32
}

fn main() {
    let solution = p2();
    println!("{}", solution);
}
