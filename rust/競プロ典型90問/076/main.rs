use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let mut a2: Vec<u64> = line2
        .split_whitespace()
        .map(|v| FromStr::from_str(&v).unwrap())
        .collect();

    // 合計値取得
    let expected_sum: u64 = a2.clone().iter().sum::<u64>() / 10;

    // 環状データを簡単に扱えるように、 2 倍に引き伸ばす
    a2.append(&mut a2.clone());

    let mut l: usize = 0;
    let mut r: usize = 1;

    while l != n && r != l + n + 1 {
        if a2[l..r].len() == 0 {
            // 範囲がゼロなら r をプラスして次へ
            r += 1;
            continue;
        }

        // 範囲の合計が全体の合計の 1/10 になるかチェック
        let local_sum: u64 = a2[l..r].iter().sum();
        if local_sum == expected_sum {
            println!("{}", "Yes");
            return;
        } else if local_sum < expected_sum {
            // 小さいなら範囲を広げる
            r += 1;
        } else if local_sum > expected_sum {
            // 大きいなら範囲を狭める
            l += 1;
        }
    }

    println!("{}", "No");
}
