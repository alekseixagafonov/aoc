use std::io::{self, BufRead};

fn sum<R: BufRead>(r: R) -> u64 {
    r.lines()
        .map(|line| {
            let s = line.unwrap();
            let mut digits = Vec::new();
            let mut k = s.len() - 12;

            for b in s.bytes() {
                while k > 0 && !digits.is_empty() && *digits.last().unwrap() < b {
                    digits.pop();
                    k -= 1;
                }
                digits.push(b);
            }

            while k > 0 {
                digits.pop();
                k -= 1;
            }

            digits.truncate(12);

            digits
                .into_iter()
                .fold(0u64, |acc, d| acc * 10 + (d - b'0') as u64)
        })
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
