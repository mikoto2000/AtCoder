use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    let s: Vec<_> = line2.trim().parse::<String>().ok().unwrap().chars().map(|v| v.to_string()).collect();

    // println!("{:?}", s);

    let mut max = 0;
    for i in 0..n {
        let left: Vec<String> = (&s[0..i]).to_vec();
        let right: Vec<String> = (&s[i..]).to_vec();

        let mut same_count = 0;
        let mut existed_same_set = HashSet::new();
        for j in left {
            for k in &right {
                if j == *k && !existed_same_set.contains(&j) {
                    same_count += 1;
                    existed_same_set.insert(j.clone());
                }
            }
        }
        max = max.max(same_count);
    }
    println!("{}", max);
}
