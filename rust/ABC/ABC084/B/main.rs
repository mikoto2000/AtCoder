fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let a: usize = FromStr::from_str(&line1[0]).unwrap();
    let b: usize = FromStr::from_str(&line1[1]).unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    line2 = line2.trim().parse::<String>().ok().unwrap();
    let chars: Vec<_> = line2.chars().collect();

    // A の範囲がすべて数字であることを確認
    if !chars[0..a].iter().all(|v| is_number(&v)) {
        println!("{}", "No");
        return;
    }

    // A の範囲の直後が '-' であることを確認
    if chars[a] != '-' {
        println!("{}", "No");
        return;
    }

    // 残りの文字が B の長さであるか確認
    let ib = &chars[a + 1..];
    if ib.len() != b {
        println!("{}", "No");
        return;
    }

    // 残りの文字列が全て数字であることを確認
    if !ib.iter().all(|v| is_number(&v)) {
        println!("{}", "No");
    } else {
        println!("{}", "Yes");
    }
}

fn is_number(c: &char) -> bool {
    *c >= '0' && *c <= '9'
}
