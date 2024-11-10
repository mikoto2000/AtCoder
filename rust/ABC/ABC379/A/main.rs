fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let abc: String = FromStr::from_str(&line1[0]).unwrap();

    let a = &abc[0..0 + 1];
    let b = &abc[1..1 + 1];
    let c = &abc[2..2 + 1];

    let bca = format!("{}{}{}", b, c, a);
    let cab = format!("{}{}{}", c, a, b);

    println!("{} {}", bca, cab);
}
