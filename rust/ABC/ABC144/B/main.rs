fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u64 = FromStr::from_str(&line1[0]).unwrap();

    for i in 1..10 {
        for j in 1..10 {
            if i * j == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("{}", "No");
}
