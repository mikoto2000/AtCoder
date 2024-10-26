fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u64 = FromStr::from_str(&line1[0]).unwrap();
    let mut m: u64 = FromStr::from_str(&line1[1]).unwrap();

    // 店情報のベクタ
    let mut ss = vec![(0, 0); n as usize];

    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();
        let line2: Vec<&str> = line2.split_whitespace().collect();
        let a: u64 = FromStr::from_str(&line2[0]).unwrap();
        let b: u64 = FromStr::from_str(&line2[1]).unwrap();

        ss[i as usize] = (a, b);
    }

    ss.sort_by(|a, b| a.cmp(b));

    // 一番安いものから順番に購入
    let mut result = 0;
    for v in ss {
        if v.1 > m {
            result += v.0 * m;
            break;
        } else {
            result += v.0 * v.1;
            m -= v.1
        }
    }
    println!("{}", result);
}
