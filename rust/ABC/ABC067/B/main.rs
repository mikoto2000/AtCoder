fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let _: u64 = FromStr::from_str(&line1[0]).unwrap();
    let k: u64 = FromStr::from_str(&line1[1]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let mut l: Vec<u64> = line2.split_whitespace().map(|v| FromStr::from_str(&v).unwrap()).collect();

    l.sort_by(|a, b| b.cmp(a));

    let mut result = 0;
    for i in 0..k {
        result += l[i as usize];
    }

    println!("{}", result);
}
