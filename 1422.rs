use std::io::BufRead;

fn main() {
    let n_k = input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut res_arr = Vec::new();
    let mut big_size = String::from("0");
    let mut all_same = true;
    for _ in 0..n_k[0] {
        let s = input();
        res_arr.push(s.clone());

        if s.len() != big_size.len() {
            all_same = false;
        }

        if s.len() > big_size.len() {
            big_size = s;
        } else if s.len() == big_size.len() {
            let len = s.len();
            let repeat = 2;
            if (s.repeat(repeat * 2)[..len * 2]).parse::<i128>().unwrap()
                > (big_size.repeat(repeat * 2)[..len * 2])
                    .parse::<i128>()
                    .unwrap()
            {
                big_size = s;
            }
        }
    }

    if n_k[0] < n_k[1] {
        for _ in 0..(n_k[1] - n_k[0]) {
            if !all_same {
                res_arr.push(big_size.clone());
            }
        }
    }

    res_arr.sort_by_key(|x| x.len());
    let len = res_arr[res_arr.len() - 1].len();
    let repeat = (len as f32 / res_arr[0].len() as f32).round() as usize + 1;

    res_arr.sort_by_key(|x| -(x.repeat(repeat * 2)[..len * 2]).parse::<i128>().unwrap());

    let mut res = String::new();
    for i in 0..res_arr.len() {
        res.push_str(&res_arr[i]);
    }
    if n_k[0] < n_k[1] {
        for _ in 0..(n_k[1] - n_k[0]) {
            if all_same {
                res = format!("{}{}", &res_arr[0], res);
            }
        }
    }
    println!("{}", res);
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
