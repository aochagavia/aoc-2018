const INPUT: &'static [u8] = include_bytes!("input");

fn p2() -> usize {
    let mut values = Vec::new();
    for b in b'a'..=b'z' {
        let filtered_input: Vec<u8> = INPUT.iter().cloned().filter(|c| b != c.to_ascii_lowercase()).collect();
        values.push(p1(&filtered_input));
    }

    *values.iter().min().unwrap()
}

fn p1(start_input: &[u8]) -> usize {
    let mut input = start_input.to_owned();
    loop {
        let mut new_input = Vec::new();

        let mut i = 0;
        while i < input.len() {
            // We need to push the last char as well
            if i == input.len() - 1 {
                new_input.push(input[i]);
                break;
            }

            let fst = input[i];
            let snd = input[i + 1];

            let same_type = fst.to_ascii_lowercase() == snd.to_ascii_lowercase();
            if same_type {
                let same_polarity = fst == snd;
                if same_polarity {
                    // Push one
                    new_input.push(fst);
                    i += 1;
                    continue;
                } else {
                    // Skip two
                    i += 2;
                    continue;
                }
            }

            // Push one
            new_input.push(fst);
            i += 1;
        }

        if new_input.len() != input.len() {
            input = new_input;
        } else {
            return input.len();
        }
    }
}

fn main() {
    let solution = p2();
    println!("{}", solution);
}
