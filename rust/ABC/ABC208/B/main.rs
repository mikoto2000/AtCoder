use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let mut p: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut coin_count = 0;
    for v in (1..10 + 1).rev() {
        let mut coin = 1;
        for i in 1..v + 1 {
            coin *= i;
        }

        // コインの価値が大きすぎるなら次のコインへ
        if coin > p {
            continue;
        }

        // コインの枚数と残金を更新
        coin_count += p / coin;
        p = p % coin;
    }

    println!("{}", coin_count);
}
