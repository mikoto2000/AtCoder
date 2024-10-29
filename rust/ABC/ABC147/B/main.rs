fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let s = line1.trim().parse::<String>().ok().unwrap();
    let s2 = s.chars().rev().collect::<String>();

    let mut count = 0;
    for i in 0..s.len() / 2 {
        if s[i..i + 1] != s2[i..i + 1] {
            count += 1;
        }
    }

    println!("{}", count);
}
