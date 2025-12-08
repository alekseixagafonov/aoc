use std::io::{self, BufRead};
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

fn sum<R: BufRead>(r: R) -> u64 {
    // читаем строки, аккуратно разворачиваем Result<String, _>
    let points: Vec<[i64; 3]> = r
        .lines()
        .filter_map(Result::ok) // берём только Ok(line)
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.split(',').map(|x| x.trim().parse::<i64>().unwrap());
            [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
        })
        .collect();

    let n = points.len();
    if n == 0 {
        return 0;
    }

    // строим все пары с расстояниями
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i][0] - points[j][0];
            let dy = points[i][1] - points[j][1];
            let dz = points[i][2] - points[j][2];
            edges.push((dx * dx + dy * dy + dz * dz, i, j));
        }
    }

    edges.sort_by_key(|e| e.0);

    // сколько пар соединять
    let k = 1000.min(edges.len());

    // конкретная реализация: QuickUnionUf с UnionBySize
    let mut uf: QuickUnionUf<UnionBySize> = QuickUnionUf::new(n);
    for &(_, a, b) in edges.iter().take(k) {
        uf.union(a, b);
    }

    // считаем размеры компонент
    let mut comp = vec![0usize; n];
    for i in 0..n {
        let root = uf.find(i);
        comp[root] += 1;
    }

    let mut sizes: Vec<usize> = comp.into_iter().filter(|&x| x > 0).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let a = *sizes.get(0).unwrap_or(&0) as u64;
    let b = *sizes.get(1).unwrap_or(&1) as u64;
    let c = *sizes.get(2).unwrap_or(&1) as u64;

    a * b * c
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
