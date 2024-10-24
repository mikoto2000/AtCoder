#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 一年生の人数 N を取得
    unsigned int n;
    cin >> n;

    // 学生のクラス・期末試験情報を取得
    pair<unsigned short, unsigned short> students[n];
    for (int i = 0; i < n; i++) {
        // クラスと期末試験の点数を取得
        unsigned short c, p;
        cin >> c >> p;
        students[i] = make_pair(c, p);
    }

    // 質問の数 Q を取得
    unsigned int q;
    cin >> q;

    // 質問の回答を計算
    for (int i = 0; i < q; i++) {
        // 始点と終点を取得
        unsigned int l, r;
        cin >> l >> r;

        // 配列はゼロ始まりなので l, r から 1 をひく
        l--; r--;

        // 1 組, 2 組それぞれの合計を計算
        unsigned long sum1 = 0, sum2 = 0;
        for (int j = l; j <= r; j++) {
            if (students[j].first == 1) {
                sum1 += students[j].second;
            } else {
                sum2 += students[j].second;
            }
        }

        // 1 組, 2 組それぞれの合計を出力
        cout << sum1 << " " << sum2 << endl;
    }

    return 0;
}


