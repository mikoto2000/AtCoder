fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: String = line1.split_whitespace().collect::<Vec<_>>().join("");

    let number: u32 = FromStr::from_str(&line1).unwrap();

    if number % 4 == 0 {
        println!("{}", "YES");
    } else {
        println!("{}", "NO");
    }
}
