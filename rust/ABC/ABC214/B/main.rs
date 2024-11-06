fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let s: usize = FromStr::from_str(&line1[0]).unwrap();
    let t: usize = FromStr::from_str(&line1[1]).unwrap();

    let mut count = 0;
    for i in 0..s + 1 {
        for j in 0..s + 1 {
            for k in 0..s + 1 {
                if i + j + k <= s && i * j * k <= t {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
