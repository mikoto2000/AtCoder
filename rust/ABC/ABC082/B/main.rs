fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let s = line1.trim().parse::<String>().ok().unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    let t = line2.trim().parse::<String>().ok().unwrap();

    let mut ss = s.chars().collect::<Vec<_>>();
    ss.sort();
    let mut tt = t.chars().collect::<Vec<_>>();
    tt.sort_by(|a, b| b.cmp(a));

    let sstr = ss.into_iter().collect::<String>();
    let tstr = tt.into_iter().collect::<String>();

    if sstr.cmp(&tstr) == std::cmp::Ordering::Less {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
