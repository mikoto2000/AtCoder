fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let x: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut years = 0;
    let mut sum = 100;

    loop {
        // 金額・年度更新
        years += 1;
        sum += sum / 100;

        // 金額確認
        if sum >= x {
            break;
        }
    }

    println!("{}", years);
}
