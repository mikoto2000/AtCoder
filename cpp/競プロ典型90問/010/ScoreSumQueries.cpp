#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 一年生の人数 N を取得
    unsigned int n;
    cin >> n;

    // 学籍番号 i 番目までの累積和をすべて記録する配列
    long long sum1[n + 1], sum2[n + 1];
    sum1[0] = 0;
    sum2[0] = 0;

    // 1 組、 2 組それぞれの累積和を計算
    for (int i = 1; i <= n; i++) {
        short c, p;
        cin >> c >> p;

        if (c == 1) {
            sum1[i] = sum1[i - 1] + p;
            sum2[i] = sum2[i - 1];
        } else {
            sum2[i] = sum2[i - 1] + p;
            sum1[i] = sum1[i - 1];
        }
    }

    // 質問の数 Q を取得
    unsigned int q;
    cin >> q;

    // 質問の回答を計算
    for (int i = 1; i <= q; i++) {
        // 始点と終点を取得
        int l, r;
        cin >> l >> r;

        // 1 組, 2 組それぞれの合計を出力
        cout << sum1[r] - sum1[l - 1] << " " << sum2[r] - sum2[l - 1] << endl;
    }

    return 0;
}

