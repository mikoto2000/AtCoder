fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u64 = FromStr::from_str(&line1[0]).unwrap();
    let y: u64 = FromStr::from_str(&line1[1]).unwrap();

    for i in 0..n + 1 {
        for j in 0..n - i + 1 {
            let k = n - i - j;
            if i * 10000 + j * 5000 + k * 1000 == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}
