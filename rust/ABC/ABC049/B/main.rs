use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let h: usize = FromStr::from_str(&line1[0]).unwrap();
    let w: usize = FromStr::from_str(&line1[1]).unwrap();

    let mut origin = vec![vec!["".to_string(); w]; h];

    for i in 0..h {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();

        for j in 0..w {
            origin[i][j] = line2[j..j + 1].to_string();
        }
    }

    for i in 0..h * 2 {
        for j in 0..w {
            print!("{}", origin[i / 2][j]);
        }
        println!();
    }
}
