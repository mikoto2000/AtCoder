use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let mut a: Vec<u64> = line1
        .split_whitespace()
        .map(|v| FromStr::from_str(&v).unwrap())
        .collect();
    a.sort();

    let mut i: usize = 0;
    let mut count = 0;

    while i < a.len() - 1 {
        if a[i] == a[i + 1 as usize] {
            i += 2;
            count += 1;
        } else {
            i += 1;
        }
    }
    println!("{}", count);
}
