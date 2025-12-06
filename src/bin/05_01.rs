use std::io::{self, BufRead};

fn sum<R: BufRead>(r: R) -> u64 {
    let mut intervals: Vec<(i64, i64)> = Vec::new();
    let mut reading_intervals = true;
    let mut fresh_count: u64 = 0;

    for line_res in r.lines() {
        let line = match line_res {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };

        if line.is_empty() {
            reading_intervals = false;
            continue;
        }

        if reading_intervals {
            let parts: Vec<_> = line.split('-').collect();
            if parts.len() != 2 {
                continue;
            }

            let a: i64 = parts[0].trim().parse().unwrap();
            let b: i64 = parts[1].trim().parse().unwrap();
            let (l, r) = if a <= b { (a, b) } else { (b, a) }; // на всякий
            intervals.push((l, r));
        } else {
            let id: i64 = line.parse().unwrap();

            if intervals.iter().any(|(l, r)| id >= *l && id <= *r) {
                fresh_count += 1;
            }
        }
    }

    fresh_count
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
