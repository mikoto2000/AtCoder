#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 数値 N, A, B を取得
    unsigned short n, a, b;
    cin >> n >> a >> b;

    // 1 以上 N 以下の整数のうち、 10 進法での各桁の和が A 以上 B 以下である者の総和を計算
    unsigned int all_sum = 0;
    for (int i = 1; i <= n; i++) {
        unsigned int digits_sum = 0;
        unsigned int tmp_number = i;

        // 各桁の和を計算
        do {
            // 一桁目を取得して digits_sum へ追加
            digits_sum += tmp_number % 10;

        } while ((tmp_number = tmp_number / 10) != 0);

        // A 以上 B 以下であるか確認
        if (digits_sum >= a && digits_sum <= b) {
            // A 以上 B 以下であれば全体の和に追加
            all_sum += i;
        }
    }

    // 結果出力
    cout << all_sum << endl;

    return 0;
}

