fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let s = line1.trim().parse::<String>().ok().unwrap();

    for i in 1..s.len() - 1 {
        let end_index = s.len() - i;
        let ss = &s[0..end_index];

        let ss_head = &ss[0..end_index / 2];
        let ss_tail = &ss[end_index / 2..];

        if ss_head == ss_tail {
            println!("{}", ss_head.len() + ss_tail.len());
            break;
        }
    }
}
