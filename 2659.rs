use std::io::{self, BufRead};
fn main() {
    let i = input();
    let mut check_num = vec![];
    let c_num = parse_clock_num(i[0], i[1], i[2], i[3]);

    for i in 1..=9 {
        for j in 1..=9 {
            for k in 1..=9 {
                for l in 1..=9 {
                    let clock_num = parse_clock_num(i, j, k, l);
                    if !check_num.contains(&clock_num) {
                        if c_num > clock_num {
                            check_num.push(clock_num)
                        }
                    }
                }
            }
        }
    }
    println!("{}", check_num.len() + 1);
}

fn sticky_int(a: u8, b: u8, c: u8, d: u8) -> u16 {
    let res = a as u16 * 1000 + b as u16 * 100 + c as u16 * 10 + d as u16;
    // println!("{res}");
    res
}

fn parse_clock_num(i: u8, j: u8, k: u8, l: u8) -> u16 {
    u16::min(
        u16::min(sticky_int(i, j, k, l), sticky_int(j, k, l, i)),
        u16::min(sticky_int(k, l, i, j), sticky_int(l, i, j, k)),
    )
}
fn input() -> Vec<u8> {
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}
