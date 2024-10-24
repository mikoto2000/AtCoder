use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u32 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let line2: Vec<&str> = line2.split_whitespace().collect();
    let a: u32 = FromStr::from_str(&line2[0]).unwrap();

    let surplus = n % 500;

    if surplus <= a {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
