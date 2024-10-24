/**
 * See: https://atcoder.jp/contests/abc268/editorial/4776
 */
#include <algorithm>
#include <cstring>
#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 人数 N を取得
    unsigned int n;
    cin >> n;

    // 料理情報を取得
    int ps[n];
    for (int i = 0; i < n; i++) {
        cin >> ps[i];
    }

    // 回転数に対して、何人満足させられたかを記録しておく配列を作成
    // manzoku_map[回転数]
    unsigned int manzoku_map[n];
    memset(manzoku_map, 0, sizeof(unsigned int) * n);

    // 料理の数だけ繰り返す
    for (int i = 0; i < n; i++) {
        // cout << i << "番目の料理" << endl;

        // 満足させられる回転数を求める
        int hito_number = ps[i];
        // cout << "hito_number" << hito_number << endl;

        // 料理のインデックス = 料理の初期位置
        // 料理の数字 = 満足する人間の位置
        // 料理の初期位置と、満足する人間の位置の差分をとって、
        // どれだけ回せば満足する人が出現するかを求める
        int rotate_count = hito_number - i;
        if (rotate_count < 0) {
            rotate_count += n;
        }
        int rotate_count_p1 = (rotate_count + 1) % n;
        int rotate_count_m1 = rotate_count - 1;
        if (rotate_count_m1 < 0) {
            rotate_count_m1 += n;
        }

        // cout << rotate_count_p1 << endl;
        // cout << rotate_count << endl;
        // cout << rotate_count_m1 << endl;

        // 満足する回転回数にプラス 1 する
        manzoku_map[rotate_count_p1] += 1;
        manzoku_map[rotate_count] += 1;
        manzoku_map[rotate_count_m1] += 1;
    }

    // for (int i = 0; i < n; i++) {
    //      cout << manzoku_map[i] << " ";
    // }
    // cout << endl;

    cout << *(max_element(manzoku_map, manzoku_map + n)) << endl;

    return 0;
}

