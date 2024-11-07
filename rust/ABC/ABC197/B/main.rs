fn main() {
    use std::str::FromStr;

    let mut line1 = String::new();
    std::io::stdin().read_line(&mut line1).ok();
    line1 = line1.trim().parse::<String>().ok().unwrap();
    let line1: Vec<&str> = line1.split_whitespace().collect();
    let h: usize = FromStr::from_str(&line1[0]).unwrap();
    let w: usize = FromStr::from_str(&line1[1]).unwrap();

    // ゼロ始まりに合わせるためにマイナス 1 する
    let x: usize = FromStr::from_str(&line1[2]).unwrap();
    let x = x - 1;
    let y: usize = FromStr::from_str(&line1[3]).unwrap();
    let y = y - 1;

    let mut map = vec![vec![".".to_string(); w]; h];

    for i in 0..h {
        let mut line2 = String::new();
        std::io::stdin().read_line(&mut line2).ok();
        let s = line2.trim().parse::<String>().ok().unwrap();

        for j in 0..w {
            if s[j..j + 1] == "#".to_string() {
                map[i][j] = "#".to_string();
            }
        }
    }

    let count = count_view_cell(&map, h, w, x, y);

    println!("{}", count);
}

fn count_view_cell(map: &Vec<Vec<String>>, h: usize, w: usize, x: usize, y: usize) -> usize {
    // 自分自身のセルを含むため初期値は 1
    let mut count = 1;

    // 上
    if (x as i64) - 1 >= 0 {
        for i in (0..x).rev() {
            if map[i][y] == "#".to_string() {
                break;
            } else {
                count += 1;
            }
        }
    }

    // 下
    if x + 1 < h {
        for i in x + 1..h {
            if map[i][y] == "#".to_string() {
                break;
            } else {
                count += 1;
            }
        }
    }

    // 右
    if y + 1 < w {
        for i in y + 1..w {
            if map[x][i] == "#".to_string() {
                break;
            } else {
                count += 1;
            }
        }
    }

    // 左
    if (y as i64) - 1 >= 0 {
        for i in (0..y).rev() {
            if map[x][i] == "#".to_string() {
                break;
            } else {
                count += 1;
            }
        }
    }

    return count;
}
