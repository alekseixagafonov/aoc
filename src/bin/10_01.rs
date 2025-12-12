use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_line(line: &str) -> Option<(u128, Vec<u128>)> {
    let s = line.trim();
    if s.is_empty() {
        return None;
    }

    // [diagram]
    let l = s.find('[')?;
    let r = s[l + 1..].find(']')? + l + 1;
    let diagram = &s[l + 1..r];

    let mut target: u128 = 0;
    let n = diagram.len();
    for (i, c) in diagram.chars().enumerate() {
        if c == '#' {
            target |= 1u128 << i;
        }
    }

    // parse all (...) before '{'
    let cut = s.find('{').unwrap_or(s.len());
    let rest = &s[r + 1..cut];

    let mut buttons = Vec::new();
    let bytes = rest.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'(' {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != b')' {
                j += 1;
            }

            let mut mask: u128 = 0;
            let inside = rest[start..j].trim();
            if !inside.is_empty() {
                for t in inside.split(',') {
                    let idx: usize = t.trim().parse().unwrap();
                    assert!(idx < n);
                    mask |= 1u128 << idx;
                }
            }
            buttons.push(mask);
            i = j + 1;
        } else {
            i += 1;
        }
    }

    Some((target, buttons))
}

fn min_presses(target: u128, buttons: &[u128]) -> usize {
    let m = buttons.len();
    let mid = m / 2;
    let (left, right) = buttons.split_at(mid);

    let mut best: HashMap<u128, usize> = HashMap::new();

    // left half
    for mask in 0u64..(1u64 << left.len()) {
        let mut x = 0u128;
        let mut cnt = 0usize;
        for i in 0..left.len() {
            if (mask >> i) & 1 == 1 {
                x ^= left[i];
                cnt += 1;
            }
        }
        best.entry(x)
            .and_modify(|v| *v = (*v).min(cnt))
            .or_insert(cnt);
    }

    let mut ans = usize::MAX;

    // right half
    for mask in 0u64..(1u64 << right.len()) {
        let mut x = 0u128;
        let mut cnt = 0usize;
        for i in 0..right.len() {
            if (mask >> i) & 1 == 1 {
                x ^= right[i];
                cnt += 1;
            }
        }
        let need = target ^ x;
        if let Some(&lcnt) = best.get(&need) {
            ans = ans.min(lcnt + cnt);
        }
    }

    ans
}

fn sum<R: BufRead>(r: R) -> i64 {
    let mut total = 0i64;

    for line in r.lines() {
        let line = line.unwrap();
        if let Some((target, buttons)) = parse_line(&line) {
            let presses = min_presses(target, &buttons);
            total += presses as i64;
        }
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
