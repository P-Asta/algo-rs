// NOT AC
use std::io::{self, BufRead};
fn main() {
    let l: usize = input().parse().unwrap();
    let mut fx_dx = input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let integral_range = input()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut min_fxdx = vec![];
    let mut current_fxdx = vec![];

    for i in 0..l / 2 {
        min_fxdx.push(fx_dx.remove(0));
        if min_fxdx.eq(&fx_dx[0..=i]) {
            current_fxdx = min_fxdx.clone();
        }
    }
    let periodic = current_fxdx.len();
    if integral_range[0] == integral_range[1] {
        print!("{res}", res = 0);
        return;
    }
    let module_first = integral_range[0] % periodic as i32;
    let module_last = integral_range[1] % periodic as i32;
    let size = (integral_range[1] - integral_range[0]) / periodic as i32;

    let mut sum = 0;
    for i in module_first..module_last + periodic as i32 {
        sum += current_fxdx[i as usize % periodic];
    }
    print!("{res}", res = sum * size);
}
fn input() -> String {
    io::stdin().lock().lines().next().unwrap().unwrap()
}
