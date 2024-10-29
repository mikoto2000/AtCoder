fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();

    let s = <u64 as FromStr>::from_str(&line1[0]);
    match s {
        Ok(s) => println!("{}", s * 2),
        Err(_) => println!("{}", "error"),
    }
}
