use std::io::{self, BufRead};

fn sum<R: BufRead>(r: R) -> i32 {
    r.lines()
        .map(|l| {
            l.unwrap()
                .bytes()
                .filter(|b| b.is_ascii_digit())
                .fold((0i32, None::<i32>), |(max, prev), b| {
                    let d = (b - b'0') as i32;
                    if let Some(p) = prev {
                        (max.max(p * 10 + d), Some(p.max(d)))
                    } else {
                        (max, Some(d))
                    }
                })
                .0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_basic() {
        let input = "123\n456\n789\n";
        let result = sum(input.as_bytes());
        assert_eq!(result, 168);
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
