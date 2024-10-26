use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let _: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let line2: Vec<&str> = line2.split_whitespace().collect();
    let t: f64 = FromStr::from_str(&line2[0]).unwrap();
    let a: f64 = FromStr::from_str(&line2[1]).unwrap();

    let mut line3 = String::new();
    std::io::stdin().read_line(&mut line3).ok();
    line3 = line3.trim().parse::<String>().ok().unwrap();
    let hs: Vec<f64> = line3.split_whitespace().map(|v| FromStr::from_str(&v).unwrap()).collect();

    let mut result = 0;
    let mut min = 100000.0;
    for (i, h) in hs.into_iter().enumerate() {
        let temp = t - h * 0.006;
        let temp = temp;
        let diff = (a - temp).abs();
        if min > diff {
            min = diff;
            result = i;
        }
    }

    println!("{}", result + 1);
}
