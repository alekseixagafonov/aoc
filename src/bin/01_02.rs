use std::io::{self, BufRead};

fn zero_hits(dial_before: i32, value: i32, dir: char) -> i32 {
    let dial_before = dial_before.rem_euclid(100);

    let d = match dir {
        'R' => (100 - dial_before) % 100,
        'L' => dial_before % 100,
        _ => panic!("Unknown direction: {}", dir),
    };

    if d == 0 {
        value / 100
    } else if value < d {
        0
    } else {
        1 + (value - d) / 100
    }
}

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
        let dir = chars.next().unwrap();
        let rest = chars.as_str();

        let value: i32 = rest.parse().expect("invalid number");

        let dial_before = dial;

        // считаем, сколько раз во время ЭТОГО поворота был 0
        count += zero_hits(dial_before, value, dir);

        // обновляем циферблат так же, как у тебя было
        if dir == 'R' {
            dial += value;
        } else if dir == 'L' {
            dial += 100 - value;
        } else {
            panic!("Unknown direction: {}", dir);
        }

        dial %= 100;
    }

    count
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
