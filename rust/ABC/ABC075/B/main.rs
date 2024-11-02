fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let h: i64 = FromStr::from_str(&line1[0]).unwrap();
    let w: i64 = FromStr::from_str(&line1[1]).unwrap();

    let mut map = vec![vec![0; w as usize]; h as usize];

    for i in 0..h {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        let s = &line2.trim().parse::<String>().ok().unwrap();

        for j in 0..s.len() {
            let j = j as i64;
            if s[(j as usize)..((j + 1) as usize)] == *"#" {
                // 地雷は -1 とする
                map[i as usize][j as usize] = -1;
                // 周り 8 マス += 1 する

                // -1 -1
                if i - 1 >= 0 && j - 1 >= 0 && map[(i - 1) as usize][(j - 1) as usize] != -1 {
                    map[(i - 1) as usize][(j - 1) as usize] += 1;
                }

                // -1 0
                if i - 1 >= 0 && map[(i - 1) as usize][j as usize] != -1 {
                    map[(i - 1) as usize][j as usize] += 1;
                }

                // -1 +1
                if i - 1 >= 0 && j + 1 < w && map[(i - 1) as usize][(j + 1) as usize] != -1 {
                    map[(i - 1) as usize][(j + 1) as usize] += 1;
                }

                // 0 -1
                if j - 1 >= 0 && map[i as usize][(j - 1) as usize] != -1 {
                    map[i as usize][(j - 1) as usize] += 1;
                }

                // 0 +1
                if j + 1 < w && map[i as usize][(j + 1) as usize] != -1 {
                    map[i as usize][(j + 1) as usize] += 1;
                }

                // +1 -1
                if i + 1 < h && j - 1 >= 0 && map[(i + 1) as usize][(j - 1) as usize] != -1 {
                    map[(i + 1) as usize][(j - 1) as usize] += 1;
                }

                // +1 0
                if i + 1 < h && map[(i + 1) as usize][j as usize] != -1 {
                    map[(i + 1) as usize][j as usize] += 1;
                }

                // +1 +1
                if i + 1 < h && j + 1 < w && map[(i + 1) as usize][(j + 1) as usize] != -1 {
                    map[(i + 1) as usize][(j + 1) as usize] += 1;
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if map[i as usize][j as usize] == -1 {
                print!("#");
            } else {
                print!("{}", map[i as usize][j as usize]);
            }
        }
        println!();
    }
}
