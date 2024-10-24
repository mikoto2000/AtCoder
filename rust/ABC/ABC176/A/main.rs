fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u32 = FromStr::from_str(&line1[0]).unwrap();
    let x: u32 = FromStr::from_str(&line1[1]).unwrap();
    let t: u32 = FromStr::from_str(&line1[2]).unwrap();

    let surplus = n % x;

    if surplus == 0 {
        println!("{}", n / x * t);
    } else {
        println!("{}", n / x * t + t);
    }
}
