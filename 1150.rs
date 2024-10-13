// NOT AC
use std::io::BufRead;
type T = usize;

fn main() {
    let i: Vec<T> = input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // 입력 받는양, 케이블 수
    let (n, k) = (i[0], i[1]);

    let mut inputs: Vec<T> = vec![];
    // index, value
    let mut diffs: Vec<(T, T)> = vec![];

    // 입력 받기
    for _ in 0..n {
        inputs.push(input().parse().unwrap());
    }

    // 각각 값의 차를 diffs에 index와 함깨 저장
    for i in 0..n {
        if i + 1 < n {
            diffs.push((i, diff(inputs[i], inputs[i + 1])));
        }
    }

    // diffs의 길이
    let diffslen = diffs.len();

    // 예외! 모두 더하는경우
    if n == k * 2 {
        let mut res = 0;
        for i in (0..diffslen).step_by(2) {
            res += diffs[i].1;
        }
        print!("{res}");
        return;
    }

    let diffs_notsort = diffs.clone();
    diffs.sort_by_key(|x| x.1);

    // diffs의 특정 index의 사용 가능여부
    let mut canuse = vec![true; diffslen];

    // 정답
    let mut res = 0;

    // 선택한 케이블 수
    let mut select_k = 0;
    for i in 0..diffslen {
        let diff = diffs[i];

        // 이미 사용된 케이블이면 넘어감
        if !canuse[diff.0] {
            continue;
        }

        if diff.0 > 0 && diff.0 + 1 < diffslen && k - select_k > 1 {
            let mut next = false;
            for j in 0..3 {
                let idx = i + 2 + j;
                if idx < diffslen
                        // 지금 확인하는 index의 양 옆이 이미 사용되는지 확인
                        && diffs[idx].0 != diffs_notsort[diff.0 - 1].0
                        && diffs[idx].0 != diffs_notsort[diff.0 + 1].0

                        // 양 옆의 값의 합이 확인하는 index보다 작거나 같은지 확인
                        && diffs_notsort[diff.0 - 1].1 + diffs_notsort[diff.0 + 1].1 <= diffs[idx].1
                {
                    canuse[diff.0] = false;
                    next = true;
                    break;
                }
            }
            if next {
                continue;
            }
        }

        res += diff.1;
        select_k += 1;

        // k개를 선택했으면 종료
        if select_k == k {
            break;
        }

        // 양 옆의 건물에 케이블을 사용할 수 없게 함(중복 방지)
        if diff.0 > 0 && canuse[diff.0 - 1] {
            canuse[diff.0 - 1] = false;
        }
        if diff.0 + 1 < diffslen && canuse[diff.0 + 1] {
            canuse[diff.0 + 1] = false;
        }
        canuse[diff.0] = false;
    }
    print!("{res}")
}

fn diff(a: T, b: T) -> T {
    if a > b {
        a - b
    } else {
        b - a
    }
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
