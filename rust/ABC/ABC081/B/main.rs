use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();

    // 配列の値を埋める
    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let mut a: Vec<u64> = line2.split_whitespace().map(|v| FromStr::from_str(&v).unwrap()).collect();


    // 2 で割れる回数をカウント
    let mut count = 0;
    loop {
        // 全ての要素が 2 で割れるかを確認
        if !(a.iter().all(|v| v % 2 == 0)) {
            break;
        }

        count += 1;

        // 全ての要素を 2 で割る
        a = a.iter().map(|v| v / 2).collect();

    }

    println!("{}", count);
}
