use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    _ = line1.trim().parse::<String>().ok().unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let h: Vec<u64> = line2
        .split_whitespace()
        .map(|v| FromStr::from_str(&v).unwrap())
        .collect();

    let mut max = 0;
    let mut num = 0;
    for v in h.into_iter() {
        if v < max {
            // do nothing
        } else {
            max = v;
            num += 1;
        }
    }

    println!("{}", num);
}
