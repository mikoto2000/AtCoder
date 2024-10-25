fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let a: f64 = FromStr::from_str(&line1[0]).unwrap();
    let b: f64 = FromStr::from_str(&line1[1]).unwrap();

    for i in 1..10000 {
        if (i as f64 * 0.08).floor() == a && (i as f64 * 0.1).floor() == b {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}
