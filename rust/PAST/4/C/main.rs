fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();
    let m: usize = FromStr::from_str(&line1[1]).unwrap();

    let mut map = vec![vec![0; m]; n];

    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        let s = line2.trim().parse::<String>().ok().unwrap();

        for j in 0..m {
            if s[j..j + 1] == "#".to_string() {
                update_map(&mut map, n, m, i as usize, j as usize);
            }
        }
    }

    // 結果表示
    for i in 0..n {
        for j in 0..m {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

fn update_map(map: &mut Vec<Vec<usize>>, h: usize, w: usize, i: usize, j: usize) {
    // 上の行
    if i as i64 - 1 >= 0 {
        //　左
        if j as i64 - 1 >= 0 {
            map[i - 1][j - 1] += 1;
        }
        //　中央
        map[i - 1][j] += 1;

        //　右
        if j + 1 < w {
            map[i - 1][j + 1] += 1;
        }
    }

    // 対象行
    //　左
    if j as i64 - 1 >= 0 {
        map[i][j - 1] += 1;
    }
    //　中央
    map[i][j] += 1;

    //　右
    if j + 1 < w {
        map[i][j + 1] += 1;
    }

    // 下の行
    if i + 1 < h {
        //　左
        if j as i64 - 1 >= 0 {
            map[i + 1][j - 1] += 1;
        }
        //　中央
        map[i + 1][j] += 1;

        //　右
        if j + 1 < w {
            map[i + 1][j + 1] += 1;
        }
    }
}
