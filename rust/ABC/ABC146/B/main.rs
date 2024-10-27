fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: u8 = FromStr::from_str(&line1[0]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    let binding = line2.trim().parse::<String>().ok().unwrap();
    let s = binding.chars();

    let s: Vec<_> = s.map(|v| rot(v, n).to_string()).collect();

    println!("{}", s.join(""));
}

fn rot(c: char, num: u8) -> char {
    let orig_c = c as u8 - 65;

    // ROT
    (((orig_c + num) % 26) + 65) as char
}
