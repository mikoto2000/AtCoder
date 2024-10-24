#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 高さ H と幅 W を取得
    unsigned short h, w;
    cin >> h >> w;

    // 「不適切な条件」が
    //     - イルミネーション全体に完全に含まれる 縦 2 × 横 2 の、4 つの LED を含む領域であって
    //     - 点灯している LED が領域内に 2 つ以上あるものが存在する
    // なので、 2 x 2 の領域でない場合には不適切にはならない。
    if (h == 1 || w == 1) {
        // 結果出力
        cout << h * w << endl;
        return 0;
    }

    // 1 行目の LED の数を計算
    unsigned short row_led_num = (w / 2) + (w % 2);

    // 1 列目の LED の数を計算
    unsigned short col_led_num = (h / 2) + (h % 2);

    // 結果出力
    cout << row_led_num * col_led_num << endl;

    return 0;
}

