use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let _: u64 = FromStr::from_str(&line1[0]).unwrap();
    let x: u64 = FromStr::from_str(&line1[1]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let l: Vec<u64> = line2
        .split_whitespace()
        .map(|v| FromStr::from_str(&v).unwrap())
        .collect();

    // 必ず 0 にはバウンドするので 1 から始める
    let mut count = 1;

    // バウンド時の原点からの距離
    let mut d_sum = 0;

    for v in l.iter() {
        if d_sum + v > x {
            break;
        }
        count += 1;
        d_sum += v;
    }

    println!("{}", count);
}
