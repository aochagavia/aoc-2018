use std::cmp;

const INPUT: &'static str = include_str!("input");

fn parse_deps() -> Vec<Vec<u8>> {
    let mut deps = Vec::new();
    let mut highest = b'A';
    for line in INPUT.lines() {
        let prev = line.as_bytes()[5];
        let next = line.as_bytes()[36];

        // println!("{} -> {}", prev as char, next as char);
        deps.push((prev - b'A', next - b'A'));

        highest = cmp::max(prev, highest);
        highest = cmp::max(next, highest);
    }

    let deps_len = (highest - b'A' + 1) as usize;
    let mut deps_per_node = vec![Vec::new(); deps_len];
    for &(prev, next) in &deps {
        // To have next, we first need prev
        deps_per_node[next as usize].push(prev);
    }

    deps_per_node
}

#[derive(Clone, Debug)]
struct Job {
    node: usize,
    finished_at: usize,
}

fn p2() -> usize {
    let deps_per_node = parse_deps();
    let node_amount = deps_per_node.len();

    let worker_amount = 5;

    // Start building the path
    let mut finished = vec![false; node_amount];
    let mut in_progress = vec![false; node_amount];
    let mut active_jobs: Vec<Option<Job>> = vec![None; worker_amount];
    let mut path = String::new();
    let mut step = 0;
    loop {
        // let idle_job = Job {
        //     node: '.',
        //     finished_at: 0,
        // };
        // println!("{}: {} {}", step,
        //     active_jobs[0].clone().unwrap_or(idle_job.clone()).node,
        //     active_jobs[1].clone().unwrap_or(idle_job.clone()).node);

        // Free the job slots of finished work
        for job_slot in &mut active_jobs {
            if let Some(job) = job_slot {
                if job.finished_at == step {
                    finished[job.node] = true;
                    path.push((job.node as u8 + b'A') as char);
                    *job_slot = None;
                }
            }
        }

        // Assign jobs to idle workers
        loop {
            let available_jobs: Vec<_> = deps_per_node
                .iter().enumerate()
                .filter(|(node, deps)| !finished[*node] && !in_progress[*node] && deps.iter().all(|&d| finished[d as usize]))
                .map(|(node, _)| node)
                .collect();

            let available_workers = active_jobs.iter_mut().filter(|job_slot| job_slot.is_none());

            let mut progress = false;
            for (worker, node) in available_workers.zip(available_jobs.into_iter()) {
                in_progress[node] = true;
                *worker = Some(Job {
                    node,
                    finished_at: step + steps_per_node(node),
                });

                progress = true;
            }

            // No idle workers OR no available jobs
            if !progress {
                if finished.iter().all(|&x| x) {
                    println!("Finish: {}", path);
                    return step;
                }

                // On to the next iteration!
                break;
            }
        }

        step += 1;
    }
}

fn steps_per_node(node: usize) -> usize {
    node + 61
}

fn p1() -> String {
    let deps_per_node = parse_deps();
    let node_amount = deps_per_node.len();

    // Build the path
    let mut finished = vec![false; node_amount];
    let mut path = String::new();
    loop {
        let mut progress = false;
        for (node, deps) in deps_per_node.iter().enumerate() {
            // We already have added this node to the path
            if finished[node] {
                continue;
            }

            if deps.iter().all(|&d| finished[d as usize]) {
                finished[node] = true;
                progress = true;
                path.push((node as u8 + b'A') as char);
                break;
            }
        }

        if !progress {
            return path;
        }
    }
}

fn main() {
    println!("Silver: {}", p1());
    println!("Gold: {}", p2());
}
