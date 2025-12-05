use std::io::{self, BufRead};

fn sum<R: BufRead>(r: R) -> u64 {
    let grid: Vec<Vec<u8>> = r
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.into_bytes())
        .collect();

    let h = grid.len();
    if h == 0 {
        return 0;
    }
    let w = grid[0].len();

    let mut accessible = 0u64;

    // 8 соседей
    let deltas: &[(i32, i32)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for y in 0..h {
        for x in 0..w {
            // считаем ТОЛЬКО рулоны бумаги
            if grid[y][x] != b'@' {
                continue;
            }

            let mut neighbours = 0;

            for (dy, dx) in deltas {
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;

                if ny < 0 || nx < 0 || ny >= h as i32 || nx >= w as i32 {
                    continue;
                }

                if grid[ny as usize][nx as usize] == b'@' {
                    neighbours += 1;
                }
            }

            if neighbours < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
