fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let x: usize = FromStr::from_str(&line1[0]).unwrap();

    let mut max = 1;
    for b in 2..x {
        for p in 2..9 {
            let beki = (b as isize).pow(p);

            if beki as usize <= x {
                max = max.max(beki);
            } else {
                break;
            }
        }
    }

    println!("{}", max);
}
