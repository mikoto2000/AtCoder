fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let s: Vec<_> = line1
        .trim()
        .parse::<String>()
        .ok()
        .unwrap()
        .chars()
        .collect();

    // 1 文字目が A でないなら WA
    if !(s[0] == 'A') {
        println!("{}", "WA");
        return;
    }

    // 2 文字目が大文字なら WA
    if !is_lower(&s[1]) {
        println!("{}", "WA");
        return;
    }

    // 3 文字目以降、最後から 2 番目までに C がひとつ含まれている
    let s2: Vec<_> = s[1..s.len() - 1].iter().filter(|v| !(is_lower(v))).collect();
    if !(s2.len() == 1 && *s2[0] == 'C') {
        println!("{}", "WA");
        return;
    }

    // 最後の文字が小文字であること
    if is_lower(&s[s.len() -1]) {
        println!("{}", "AC");
    } else {
        println!("{}", "WA");
    }


}

fn is_lower(c: &char) -> bool {
    *c >= 'a' && *c <= 'z'
}
