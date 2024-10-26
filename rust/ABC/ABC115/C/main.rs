fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();
    let k: usize = FromStr::from_str(&line1[1]).unwrap();

    let mut h = vec![0; n];

    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();
        let line2: Vec<&str> = line2.split_whitespace().collect();
        h[i] = FromStr::from_str(&line2[0]).unwrap();
    }

    h.sort();

    let mut min = 1000000000;
    for i in k - 1..n {
        let diff = h[i] - h[i - (k - 1)];
        if min > diff {
            min = diff
        }
    }

    println!("{}", min);
}
