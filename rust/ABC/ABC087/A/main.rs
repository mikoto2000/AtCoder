    use std::str::FromStr;
fn main() {

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let x: u32 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let line2: Vec<&str> = line2.split_whitespace().collect();
    let a: u32 = FromStr::from_str(&line2[0]).unwrap();

    let mut line3 = String::new();
    std::io::stdin().read_line(&mut line3).ok();
    line3 = line3.trim().parse::<String>().ok().unwrap();
    let line3: Vec<&str> = line3.split_whitespace().collect();
    let b: u32 = FromStr::from_str(&line3[0]).unwrap();

    // ケーキを購入
    let x = x - a;

    // ドーナツをできるだけ購入したあまり
    let result = x % b;

    println!("{}", result);

}
