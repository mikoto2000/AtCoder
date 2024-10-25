fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let a: u64 = FromStr::from_str(&line1[0]).unwrap();
    let b: u64 = FromStr::from_str(&line1[1]).unwrap();

    let mut count = 0;

    for v in a..b + 1 {
        // 数値を文字列へ変換
        let v_string = v.to_string();

        // 逆順へ変換
        let rv_string = v_string.chars().rev().collect::<String>();

        if v_string == rv_string {
            count += 1;
        }
    }

    println!("{}", count);
}
