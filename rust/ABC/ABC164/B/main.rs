use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let mut a: i64 = FromStr::from_str(&line1[0]).unwrap();
    let b: i64 = FromStr::from_str(&line1[1]).unwrap();
    let mut c: i64 = FromStr::from_str(&line1[2]).unwrap();
    let d: i64 = FromStr::from_str(&line1[3]).unwrap();

    loop {
        // 高橋君の攻撃
        c -= b;
        if c <= 0 {
            println!("{}", "Yes");
            break;
        }

        // 青木君の攻撃
        a -= d;
        if a <= 0 {
            println!("{}", "No");
            break;
        }
    }
}
