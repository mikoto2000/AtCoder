fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let n: usize = FromStr::from_str(&line1[0]).unwrap();

    let mut map = vec![vec!["".to_string(); 2 * n - 1]; n];

    for i in 0..n {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        let s = line2.trim().parse::<String>().ok().unwrap();

        for j in 0..2 * n - 1 {
            map[i][j] = s[j..j + 1].to_string();
        }
    }

    // 操作
    for i in (0..n).rev() {
        for j in 1..2 * n - 1 {
            //if i + 1 < n && j + 1 < 2 * n - 1 {
            //    println!(
            //        "{} {} {}",
            //        map[i + 1][j - 1],
            //        map[i + 1][j],
            //        map[i + 1][j + 1]
            //    );
            //}
            if i + 1 < n
                && j + 1 < 2 * n - 1
                && map[i][j] == "#".to_string()
                && (map[i + 1][j - 1] == "X".to_string()
                    || map[i + 1][j] == "X".to_string()
                    || map[i + 1][j + 1] == "X".to_string())
            {
                map[i][j] = "X".to_string();
            }
        }
    }

    for i in 0..n {
        for j in 0..2 * n - 1 {
            print!("{}", map[i][j]);
        }
        println!();
    }
}
