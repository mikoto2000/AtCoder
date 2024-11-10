use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let _: u64 = FromStr::from_str(&line1[0]).unwrap();
    let k: u64 = FromStr::from_str(&line1[1]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    let s = line2.trim().parse::<String>().ok().unwrap();

    let mut search_word = "".to_string();
    for _ in 0..k {
        search_word += &"O".to_string();
    }

    let count = s.match_indices(&search_word).count();

    println!("{}", count);
}
