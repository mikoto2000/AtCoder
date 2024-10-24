fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let a: i32 = FromStr::from_str(&line1[0]).unwrap();
    let b: i32 = FromStr::from_str(&line1[1]).unwrap();

    let mut results = vec![0; 3];

    results[0] = a + b;
    results[1] = a - b;
    results[2] = a * b;

    println!("{}", results.iter().max().unwrap());
}
