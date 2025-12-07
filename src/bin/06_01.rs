use std::io::{self, BufRead};

fn sum<R: BufRead>(r: R) -> i64 {
    let mut lines: Vec<String> = Vec::new();

    for line in r.lines() {
        let line = line.unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        lines.push(trimmed.to_string());
    }

    let opt_line = lines.pop().expect("no ops line");
    let opt: Vec<char> = opt_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let mut matrix: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let row: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let cols = matrix[0].len();

    let mut results: Vec<i64> = Vec::with_capacity(cols);

    for col in 0..cols {
        let op = opt[col];

        match op {
            '+' => {
                let mut acc = 0;
                for row in &matrix {
                    acc += row[col];
                }
                results.push(acc);
            }
            '*' => {
                let mut acc = 1;
                for row in &matrix {
                    acc *= row[col];
                }
                results.push(acc);
            }
            _ => panic!(),
        }
    }

    let total: i64 = results.iter().sum();
    total
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{:?}", result);
    Ok(())
}
