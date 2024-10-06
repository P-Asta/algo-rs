use std::{collections::HashMap, io::BufRead};

fn main() {
    let mut buf = String::new();
    let mut hash: HashMap<String, String> = HashMap::new();
    let wtf = input();
    let mut a = wtf.split_whitespace().map(|x| x.parse().unwrap());
    for _ in 0..a.next().unwrap() {
        let wtf = input();
        let mut a = wtf.split_whitespace().map(|x| x.to_string());
        hash.insert(a.next().unwrap(), a.next().unwrap());
    }
    for _ in 0..a.next().unwrap() {
        let a = input();
        buf.push_str(&format!("{}\n", hash.get(&a).unwrap()));
    }
    print!("{}", buf);
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
