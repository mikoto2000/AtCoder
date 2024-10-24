fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut sum = 0;

    for v in 1..n + 1 {
        if !(v % 3 == 0 || v % 5 == 0) {
            sum += v;
        }
    }

    println!("{}", sum);
}
