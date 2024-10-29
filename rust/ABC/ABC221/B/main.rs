fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    let s = line1.trim().parse::<String>().ok().unwrap();

    let mut line2 = String::new();
    std::io::stdin().read_line(&mut line2).ok();
    let t = line2.trim().parse::<String>().ok().unwrap();

    if s == t {
        println!("{}", "Yes");
        return;
    }

    let t: Vec<_> = t.chars().collect();

    for i in 0..t.len() - 1 {
        let t2 = swap(&t, i);

        if s == t2.iter().collect::<String>() {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}

fn swap(str: &Vec<char>, i: usize) -> Vec<char> {
    let mut str = str.clone();
    let tmp = str[i];
    str[i] = str[i + 1];
    str[i + 1] = tmp;
    str
}
