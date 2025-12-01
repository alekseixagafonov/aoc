use std::io::{self, BufRead};

fn sum<R: BufRead>(reader: R) -> i32 {
    let mut count = 0;
    let mut dial = 50;

    for line_result in reader.lines() {
        let line = line_result.expect("failed to read line");
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut chars = line.chars();
        let dir = chars.next().unwrap(); // 'R' или 'L'
        let rest = chars.as_str(); // остальная часть строки — число

        let value: i32 = rest.parse().expect("invalid number");

        if dir == 'R' {
            dial += value;
        } else if dir == 'L' {
            dial += 100 - value;
        } else {
            panic!("Unknown direction: {}", dir);
        }

        dial %= 100;

        if dial == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            3,
            sum("\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
            .as_bytes())
        );
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
