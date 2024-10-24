#include <algorithm>
#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 行の数 H と列の数 W を取得
    unsigned short h, w;
    cin >> h >> w;

    // 行列と、行・列ごとの合計を格納する配列
    unsigned int a[h][w];
    unsigned int sum_row[h], sum_col[w];

    // ゼロ埋め
    fill_n(sum_row, h, 0);
    fill_n(sum_col, w, 0);

    // 行と列の合計を計算
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < w; j++) {
            cin >> a[i][j];

            // 行の合計更新
            sum_row[i] += a[i][j];

            // 列の合計更新
            sum_col[j] += a[i][j];
        }
    }

    // 結果出力
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < w - 1; j++) {
            cout << (sum_row[i] + sum_col[j] - a[i][j]) << " ";
        }
        cout << (sum_row[i] + sum_col[w - 1] - a[i][w - 1]) << endl;
    }

    return 0;
}

