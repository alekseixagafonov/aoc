use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    visiting: &mut HashSet<String>,
) -> i64 {
    if node == "out" {
        return 1;
    }

    if !visiting.insert(node.to_string()) {
        return 0;
    }

    let mut total = 0i64;
    if let Some(nexts) = graph.get(node) {
        for next in nexts {
            total += count_paths(graph, next, visiting);
        }
    }

    visiting.remove(node);
    total
}

fn sum<R: BufRead>(r: R) -> i64 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in r.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // ожидаем формат: "name: a b c"
        let (from, rest) = match line.split_once(':') {
            Some(v) => v,
            None => continue, // или panic!("bad line: {line}");
        };

        let from = from.trim().to_string();
        let outs: Vec<String> = rest
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect();

        graph.insert(from, outs);
    }

    let mut visiting = HashSet::new();
    count_paths(&graph, "you", &mut visiting)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let result = sum(reader);

    println!("{result}");
    Ok(())
}
