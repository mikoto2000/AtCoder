use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();
    let d: f64 = FromStr::from_str(&line1[1]).unwrap();

    let mut a: Vec<f64> = vec![0.0; n];
    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();
        let line2: Vec<&str> = line2.split_whitespace().collect();
        let x: f64 = FromStr::from_str(&line2[0]).unwrap();
        let y: f64 = FromStr::from_str(&line2[1]).unwrap();

        a[i as usize] = (x.powf(2.0) + y.powf(2.0)).sqrt();
    }

    let a: Vec<_> = a.iter().filter(|v| **v <= d).collect();

    println!("{}", a.len());
}
