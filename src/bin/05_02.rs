use std::io::{self, BufRead};

fn sum<R: BufRead>(r: R) -> u64 {
    let mut intervals: Vec<(i64, i64)> = Vec::new();

    for line in r.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let mut parts = line.split('-');
        let a: i64 = parts.next().unwrap().trim().parse().unwrap();
        let b: i64 = parts.next().unwrap().trim().parse().unwrap();
        let (start, end) = if a < b { (a, b) } else { (b, a) };
        intervals.push((start, end));
    }

    if intervals.is_empty() {
        return 0;
    }

    intervals.sort_by_key(|(start, _)| *start);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    let mut current = intervals[0];

    for &(s, e) in &intervals[1..] {
        if s <= current.1 {
            if e > current.1 {
                current.1 = e;
            }
        } else {
            merged.push(current);
            current = (s, e);
        }
    }
    merged.push(current);

    let mut total: u64 = 0;
    for (s, e) in merged {
        total += (e - s + 1) as u64;
    }

    total
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
