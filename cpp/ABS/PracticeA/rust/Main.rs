use std::str::FromStr;

fn main() {

    // 標準入力全部まとめて `all_stdio` に入る。
    // let all_stdio = read_to_string(stdin()).unwrap();
    // println!("{}", all_stdio);

    // 1 行目
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let a: u32 = FromStr::from_str(&line1).unwrap();

    // 2 行目
    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let line2: Vec<&str> = line2.split_whitespace().collect();
    let b: u32 = FromStr::from_str(&line2[0]).unwrap();
    let c: u32 = FromStr::from_str(&line2[1]).unwrap();

    // 3 行目
    let mut line3 = String::new();
    std::io::stdin().read_line(&mut line3).ok();
    let s = line3.trim().parse::<String>().ok().unwrap();

    println!("{} {}", a + b + c, s);
}
