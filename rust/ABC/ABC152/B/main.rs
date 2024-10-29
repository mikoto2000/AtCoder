fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let a: u64 = FromStr::from_str(&line1[0]).unwrap();
    let b: u64 = FromStr::from_str(&line1[1]).unwrap();

    let mut a_str = a.to_string();
    let mut b_str = b.to_string();

    if a_str.cmp(&b_str) == std::cmp::Ordering::Less {
        for _ in 1..b {
            a_str += &a.to_string();
        }
        println!("{}", a_str);
    } else {
        for _ in 1..a {
            b_str += &b.to_string();
        }
        println!("{}", b_str);
    }
}
