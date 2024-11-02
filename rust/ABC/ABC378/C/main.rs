use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let _: i64 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let a: Vec<i64> = line2
        .split_whitespace()
        .map(|v| FromStr::from_str(&v).unwrap())
        .collect();

    let mut max_index = HashMap::new();
    let mut b: Vec<i64> = vec![-1; a.len()];

    for i in 0..a.len() {
        if !max_index.contains_key(&a[i]) {
            max_index.insert(a[i], i);
            continue;
        } else {
            b[i] = *max_index.get(&a[i]).unwrap() as i64 + 1;
            max_index.insert(a[i], i);
        }
    }

    println!(
        "{}",
        b.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
