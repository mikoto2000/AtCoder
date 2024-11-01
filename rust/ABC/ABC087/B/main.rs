fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let a: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let line2: Vec<&str> = line2.split_whitespace().collect();
    let b: u64 = FromStr::from_str(&line2[0]).unwrap();

    let mut line3 = String::new();
    std::io::stdin().read_line(&mut line3).ok();
    line3 = line3.trim().parse::<String>().ok().unwrap();
    let line3: Vec<&str> = line3.split_whitespace().collect();
    let c: u64 = FromStr::from_str(&line3[0]).unwrap();

    let mut line4 = String::new();
    std::io::stdin().read_line(&mut line4).ok();
    line4 = line4.trim().parse::<String>().ok().unwrap();
    let line4: Vec<&str> = line4.split_whitespace().collect();
    let x: u64 = FromStr::from_str(&line4[0]).unwrap();


    let mut count = 0;
    for i in 0..a + 1 {
        for j in 0..b + 1 {
            for k in 0..c + 1 {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
