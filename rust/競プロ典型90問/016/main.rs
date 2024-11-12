fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: i64 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let line2: Vec<&str> = line2.split_whitespace().collect();
    let a: i64 = FromStr::from_str(&line2[0]).unwrap();
    let b: i64 = FromStr::from_str(&line2[1]).unwrap();
    let c: i64 = FromStr::from_str(&line2[2]).unwrap();

    let mut min = 10000;
    for i in 0..10000 {
        for j in 0..10000 {
            let tmp = n - (a * i + b * j);
            if tmp % c == 0 && tmp >= 0 {
                min = min.min(i + j + tmp / c);
            } else {
                continue;
            }
        }
    }

    println!("{}", min);
}
