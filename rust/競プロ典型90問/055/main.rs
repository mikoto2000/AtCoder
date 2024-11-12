fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();
    let p: usize = FromStr::from_str(&line1[1]).unwrap();
    let q: usize = FromStr::from_str(&line1[2]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let a: Vec<usize> = line2
        .split_whitespace()
        .map(|v| FromStr::from_str(&v).unwrap())
        .collect();

    // 全探索
    // オーバーフロー抑制のため、各値の余りを出してから掛け算する
    let mut count = 0;
    for i in 0..n - 4 {
        for j in i + 1..n - 3 {
            for k in j + 1..n - 2 {
                for l in k + 1..n - 1 {
                    for m in l + 1..n {
                        if (a[i] % p * a[j] % p * a[k] % p * a[l] % p * a[m] % p) == q {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
