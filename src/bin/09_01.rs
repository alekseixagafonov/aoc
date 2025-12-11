use std::io::{self, BufRead};

fn sum<R: BufRead>(r: R) -> i64 {
    let points = r
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut best_area = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let width = (x1 - x2).abs() + 1;
            let height = (y1 - y2).abs() + 1;
            let area = width * height;

            best_area = best_area.max(area);
        }
    }

    best_area
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
