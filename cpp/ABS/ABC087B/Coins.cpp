#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 500 円玉の枚数 A, 100 円玉の枚数 B, 50 円玉の枚数 C, 目標合計金額 X を取得
    unsigned short a, b, c, x;
    cin >> a >> b >> c >> x;

    // x を満たす組み合わせを数えるためのカウンタ
    unsigned int count = 0;

    // 各コインの枚数を総当たりでチェック
    for (int i = 0; i <= a; i++) {
        unsigned int sum_a = i * 500;
        for (int j = 0; j <= b; j++) {
            unsigned int sum_b = sum_a + j * 100;
            for (int k = 0; k <= c; k++) {
                // 合計チェック
                unsigned int sum_c = sum_b + k * 50;
                if (sum_c == x) {
                    count++;
                }
                // これ以上 50 円玉を積んでも x になることが無いので break
                if (sum_c + 50 > x) break;
            }
            // これ以上 100 円玉を積んでも x になることが無いので break
            if (sum_b + 100 > x) break;
        }
        // これ以上 500 円玉を積んでも x になることが無いので break
        if (sum_a + 500 > x) break;
    }

    // 結果出力
    cout << count << endl;

    return 0;
}

