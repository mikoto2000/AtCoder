fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    _ = line1.trim().parse::<String>().ok().unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let mut a: Vec<u32> = line2
        .split_whitespace()
        .map(|v| FromStr::from_str(&v).unwrap())
        .collect();

    a.sort();

    for (i, v) in a.into_iter().enumerate() {
        if (i + 1) as u32 != v {
            println!("{}", "No");
            return;
        }
    }

    println!("{}", "Yes");
}
