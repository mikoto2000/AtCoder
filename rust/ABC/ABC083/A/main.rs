use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u32 = FromStr::from_str(&line1[0]).unwrap();
    let a: u32 = FromStr::from_str(&line1[1]).unwrap();
    let b: u32 = FromStr::from_str(&line1[2]).unwrap();

    let mut all_sum = 0;
    for n in 1..n + 1 {
        // 各桁の合計を計算
        let mut sum = 0;
        let mut target = n;
        while target != 0 {
            // 一桁目を抽出して合計に追加
            let digit = target % 10;
            sum += digit;

            target = target / 10;
        }

        // A 以上 B 以下であるか判定
        if sum >= a && sum <= b {
            all_sum += n;
        }
    }

    println!("{}", all_sum);
}
