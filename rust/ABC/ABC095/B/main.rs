fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();
    let mut x: u64 = FromStr::from_str(&line1[1]).unwrap();


    // ドーナツの材料消費量を配列に格納
    let mut m = vec![0; n];
    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();
        let v: u64 = FromStr::from_str(&line2).unwrap();

        m[i] = v;
    }

    // 全てのドーナツをひとつずつ作る
    x -= m.iter().sum::<u64>();

    // 最小の材料のドーナツを作れるだけ作る
    let min = m.iter().min().unwrap();

    println!("{}", m.len() as u64 + x / min);
}
