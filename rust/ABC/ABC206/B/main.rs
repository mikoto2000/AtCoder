fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut sum = 0;
    let mut day = 0;
    loop {
        // 日付・金額更新
        day += 1;
        sum += day;

        // N 円以上を確認した
        if sum >= n {
            break;
        }
    }

    println!("{}", day);
}
