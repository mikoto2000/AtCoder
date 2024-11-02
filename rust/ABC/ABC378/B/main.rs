use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u64 = FromStr::from_str(&line1[0]).unwrap();

    let mut gomi = HashMap::new();

    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();
        let line2: Vec<&str> = line2.split_whitespace().collect();
        let q: u64 = FromStr::from_str(&line2[0]).unwrap();
        let r: u64 = FromStr::from_str(&line2[1]).unwrap();

        gomi.insert(i + 1, (q, r));
    }

    let mut line3 = String::new();
    std::io::stdin().read_line(&mut line3).ok();
    line3 = line3.trim().parse::<String>().ok().unwrap();
    let line3: Vec<&str> = line3.split_whitespace().collect();
    let q: u64 = FromStr::from_str(&line3[0]).unwrap();

    for _ in 0..q {
        let mut line4 = String::new();
        std::io::stdin().read_line(&mut line4).ok();
        line4 = line4.trim().parse::<String>().ok().unwrap();
        let line4: Vec<&str> = line4.split_whitespace().collect();
        let t: u64 = FromStr::from_str(&line4[0]).unwrap();
        let d: u64 = FromStr::from_str(&line4[1]).unwrap();

        let (q, r) = gomi.get(&t).unwrap();

        // 日付を q で割った余りが r
        let x = d % q;
        let additional_day = if x <= *r {
            *r - x
        } else {
            q - (x - *r)
        };

        println!("{}", d + additional_day);
    }
}
