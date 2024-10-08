// NOT AC
use std::io::BufRead;

fn main() {
    let mut buf = String::new();
    let n = input().parse::<u8>().unwrap();
    for i in 1..=n {
        buf.push_str(&format!("Case #{i}: {}\n", calc()));
    }
    print!("{buf}");
}

fn calc() -> isize {
    let i: Vec<_> = input()
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    let mut res = 0;
    let mut bag = i[0];

    // 값 / 크기 를 하여 크기가 1일때의 가치를 구한다.
    let emerald = i[2] as f32 / i[1] as f32;
    let sapphires = i[4] as f32 / i[3] as f32;

    // 크기가 1일때의 가치를 비교하여 가치가 높은 순으로 정렬한다.
    let size_value = if emerald > sapphires {
        [(i[1], i[2]), (i[3], i[4])]
    } else if emerald == sapphires {
        if bag / i[2] > bag / i[4] {
            [(i[1], i[2]), (i[3], i[4])]
        } else {
            [(i[3], i[4]), (i[1], i[2])]
        }
    } else {
        [(i[3], i[4]), (i[1], i[2])]
    };
    println!("{:?}", size_value);

    // 크기가 큰 순서대로 가방에 담는다.
    res += size_value[0].1 * (bag / size_value[0].0);
    bag = bag % size_value[0].0;
    res += size_value[1].1 * (bag / size_value[1].0);

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
