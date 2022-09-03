#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // お札の枚数 N と合計金額 Y を取得
    unsigned short n;
    unsigned int y;
    cin >> n >> y;

    // 10000 円、 5000 円、 1000 円の組み合わせを全探索
    unsigned short i, j, k;
    bool found = false;
    for (k = 0; k <= n; k++) {
        unsigned int sum_1000 = 1000 * k;
        // 1000 だけで金額・最大枚数が超えないかを確認
        if (sum_1000 > y) break;
        if (k > n) break;

        for (j = 0; j <= n; j++) {
            unsigned int sum_5000 = sum_1000 + 5000 * j;

            // 1000 と 5000 の組み合わせで金額・最大枚数が超えないかを確認
            if (sum_5000 > y) break;
            if (k + j > n) break;

            for (i = 0; i <= n; i++) {
                unsigned int sum_10000 = sum_5000 + 10000 * i;

                // 1000, 5000 と 10000 の組み合わせで金額・最大枚数が超えないかを確認
                if (sum_10000 > y) break;
                if (k + j + i > n) break;

                // 合計値・合計枚数チェック
                if (sum_10000 == y && k + j + i == n) {
                    found = true;
                    break;
                }
            }

            // 合計金額を満たす組み合わせが見つかっていたら続きを行わない
            if (found) break;
        }

        // 合計金額を満たす組み合わせが見つかっていたら続きを行わない
        if (found) break;
    }

    // 標準出力
    if (found) {
        // 見つかった
        cout << i << " " << j << " " << k << endl;
    } else {
        // 見つからなかった
        cout << "-1 -1 -1" << endl;
    }

    return 0;
}

