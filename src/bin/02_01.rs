use std::io::{self, BufRead};

fn is_invalid_id(id: i64) -> bool {
    let s = id.to_string();

    // длина должна быть чётной (одна и та же последовательность дважды)
    if s.len() % 2 == 1 {
        return false;
    }

    let (first, second) = s.split_at(s.len() / 2);
    first == second
}

fn sum(reader: impl BufRead) -> i64 {
    let mut sum: i64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        for range in line.split(',') {
            let range = range.trim();
            if range.is_empty() {
                continue;
            }

            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();

            for id in start..=end {
                if is_invalid_id(id) {
                    sum += id;
                }
            }
        }
    }

    sum
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
