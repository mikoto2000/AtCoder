fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let k: i64 = FromStr::from_str(&line1[0]).unwrap();
    let s: i64 = FromStr::from_str(&line1[1]).unwrap();

    let mut count = 0;
    for x in 0..k + 1 {
        for y in 0..k + 1 {
            let z = s - (x + y);
            if z >= 0 && z <= k {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
