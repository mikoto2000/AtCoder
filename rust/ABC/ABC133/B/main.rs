use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();
    let d: usize = FromStr::from_str(&line1[1]).unwrap();

    let mut xs = vec![vec![0; d]; n];

    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();
        let x: Vec<i64> = line2
            .split_whitespace()
            .map(|v| <i64 as FromStr>::from_str(&v).unwrap())
            .collect();
        xs[i] = x;
    }

    let mut count = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {

            let distance = calc_distance(d, &xs[i], &xs[j]);

            if distance.fract() == 0.0 {
                count += 1;
            }

        }
    }

    println!("{}", count);
}

fn calc_distance(d: usize, xs1: &Vec<i64>, xs2: &Vec<i64>) -> f64 {
    let mut distance = 0.0;

    for i in 0..d {
        distance += (xs1[i] as f64  - xs2[i] as f64 ).powf(2.0);
    }

    return distance.sqrt();
}
