fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    _ = line1.trim().parse::<String>().ok().unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let mut d: Vec<u64> = line2
        .split_whitespace()
        .map(|v| FromStr::from_str(&v).unwrap())
        .collect();

    d.sort();

    let median = d[d.len() / 2 - 1];
    let median_next = d[d.len() / 2];

    if median == median_next {
        // 次の数が同じ数だと、等しくならない
        println!("{}", 0);
    } else {
        println!("{}", median_next - median);
    }
}
