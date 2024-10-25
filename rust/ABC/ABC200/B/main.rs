use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let mut n: u64 = FromStr::from_str(&line1[0]).unwrap();
    let k: u64 = FromStr::from_str(&line1[1]).unwrap();

    for _ in 0..k {
        if n % 200 == 0 {
            n = n / 200;
        } else {
            // n の後ろに 200 を付ける
            n = n * 1000 + 200;
        }
    }

    println!("{}", n);
}
