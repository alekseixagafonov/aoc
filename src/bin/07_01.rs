use std::collections::HashSet;
use std::io::{self, BufRead};

fn sum<R: BufRead>(r: R) -> i64 {
    let grid: Vec<Vec<u8>> = r
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.trim_end().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| line.into_bytes())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let h = grid.len();
    let w = grid[0].len();
    if w == 0 {
        return 0;
    }

    let (start_row, start_col) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == b'S').map(|c| (r, c)))
        .expect("no source 'S' in grid");

    let initial_beams: HashSet<usize> = [start_col].into_iter().collect();
    let initial_state = (0usize, initial_beams);

    let (split_count, _final_beams) =
        (start_row..(h - 1)).fold(initial_state, |(split_count, current_beams), r| {
            let (new_splits, next_beams) = current_beams.iter().fold(
                (0usize, HashSet::new()),
                |(local_splits, mut next), &c| {
                    let cell_below = grid[r + 1][c];

                    if cell_below == b'^' {
                        let splits = local_splits + 1;

                        if c > 0 {
                            next.insert(c - 1);
                        }
                        if c + 1 < w {
                            next.insert(c + 1);
                        }

                        (splits, next)
                    } else {
                        next.insert(c);
                        (local_splits, next)
                    }
                },
            );

            (split_count + new_splits, next_beams)
        });

    split_count as i64
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{:?}", result);
    Ok(())
}
