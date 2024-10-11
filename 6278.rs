// https://www.acmicpc.net/problem/6278
use std::io::BufRead;

fn main() {
    let mut buf = String::new();
    let n = input().parse::<u8>().unwrap();
    for i in 1..=n {
        buf.push_str(&format!("Case #{i}: {}\n", calc(n)));
    }
    print!("{buf}");
}

fn calc(n: u8) -> usize {
    let i: Vec<_> = input()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let bag = i[0];
    let (s1, v1, s2, v2) = (i[1], i[2], i[3], i[4]);

    let mut res = 0;
    for i in 0..(100_000_000 / n as usize).min(bag / s1) + 1 {
        let remain = bag - s1 * i;
        res = res.max(v1 * i + v2 * (remain / s2));
    }

    for i in 0..(100_000_000 / n as usize).min(bag / s2) + 1 {
        let remain = bag - s2 * i;
        res = res.max(v2 * i + v1 * (remain / s1));
    }

    res
}

fn input() -> String {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .to_string()
}

