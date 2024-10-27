use std::collections::HashSet;

fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: i128 = FromStr::from_str(&line1[0]).unwrap();
    let m: i128 = FromStr::from_str(&line1[1]).unwrap();

    // 「置けない場所の座標」を格納するセット
    let mut no_put = HashSet::new();

    for _ in 0..m {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        line2 = line2.trim().parse::<String>().ok().unwrap();
        let line2: Vec<&str> = line2.split_whitespace().collect();
        let i: i128 = <i128 as FromStr>::from_str(&line2[0]).unwrap() - 1;
        let j: i128 = <i128 as FromStr>::from_str(&line2[1]).unwrap() - 1;

        // # ならそのマスと移動できる範囲を塗りつぶす
        no_put.insert((i, j));

        // -2 1
        if i - 2 >= 0 && j + 1 < n {
            no_put.insert((i - 2, j + 1));
        }

        // -2 -1
        if i - 2 >= 0 && j - 1 >= 0 {
            no_put.insert((i - 2, j - 1));
        }

        // -1 2
        if i - 1 >= 0 && j + 2 < n {
            no_put.insert((i - 1, j + 2));
        }

        // -1 -2
        if i - 1 >= 0 && j - 2 >= 0 {
            no_put.insert((i - 1, j - 2));
        }

        // 1 2
        if i + 1 < n && j + 2 < n {
            no_put.insert((i + 1, j + 2));
        }

        // 1 -2
        if i + 1 < n && j - 2 >= 0 {
            no_put.insert((i + 1, j - 2));
        }

        // 2 1
        if i + 2 < n && j + 1 < n {
            no_put.insert((i + 2, j + 1));
        }

        // 2 -1
        if i + 2 < n && j - 1 >= 0 {
            no_put.insert((i + 2, j - 1));
        }
    }

    // true の数を数える
    let count = (n * n) - no_put.len() as i128;

    println!("{}", count);
}
