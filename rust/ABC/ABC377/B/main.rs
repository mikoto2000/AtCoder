fn main() {
    // 盤面を表す 2 次元ベクタ
    // true が「置ける場所」, false が「置けない場所」とする
    let mut m = vec![vec![true; 8]; 8];

    for i in 0..8 {
        let mut line1 = String::new();
        std::io::stdin().read_line(&mut line1).ok();
        line1.char_indices().any(|(index, c)| {
            if c == '#' {
                // # なら行列を塗りつぶす
                for k in 0..8 {
                    m[k][index] = false;
                }

                for k in 0..8 {
                    m[i][k] = false;
                }
            }
            false
        });
    }

    // true の数を数える
    let mut count = 0;
    for i in 0..8 {
        for j in 0..8 {
            if m[i][j] {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
