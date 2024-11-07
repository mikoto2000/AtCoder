use std::str::FromStr;
fn main() {
    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let h: usize = FromStr::from_str(&line1[0]).unwrap();
    let w: usize = FromStr::from_str(&line1[1]).unwrap();

    let mut map = vec![vec![".".to_string(); w]; h];

    for i in 0..h {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        let a = line2.trim().parse::<String>().ok().unwrap();

        for j in 0..w {
            map[i][j] = a[j..j + 1].to_string();
        }
    }

    // 行削除
    // 表示させないセルは `x` で埋めることとする
    for i in 0..h {
        if map[i].iter().all(|v| *v == ".".to_string()) {
            map[i] = vec!["x".to_string(); w];
        }
    }

    // 列削除
    // 表示させないセルは `x` で埋めることとする
    for i in 0..w {
        let mut is_all = true;
        for j in 0..h {
            if map[j][i] != ".".to_string() && map[j][i] != "x".to_string(){
                is_all = false;
                break;
            }
        }
        if is_all {
            for j in 0..h {
                map[j][i] = "x".to_string();
            }
        }
    }

    // 結果表示
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == "x".to_string() {
                continue;
            }
            print!("{}", map[i][j]);
        }
        if !map[i].iter().all(|v| *v == "x".to_string()) {
            println!();
        }
    }
}
