fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let p: Vec<u8> = line1.iter().map(|v| FromStr::from_str(&v).unwrap()).collect();

    for v in p {
        let c: char = (v + 96) as char;
        print!("{}", c);
    }
    println!();
}
