fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();

    // 山の配列
    let mut m = vec![("".to_string(), 0); n];

    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();
        let line2: Vec<&str> = line2.split_whitespace().collect();
        let name: String = (&line2[0]).to_string();
        let height: u64 = FromStr::from_str(&line2[1]).unwrap();

        m[i] = (name, height);
    }

    m.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{}", m[1].0);
}
