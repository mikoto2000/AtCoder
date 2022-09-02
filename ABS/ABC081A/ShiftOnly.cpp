#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 正の整数 N の数を取得
    unsigned short n;
    cin >> n;

    // 整数をすべて取得
    unsigned int a[n];
    for (int i = 0; i < n; i++) {
        cin >> a[i];
    }

    // 操作可能な回数を求める
    unsigned int count = 0;
    while(true) {
        // 2 で割れないものがあったフラグ
        bool failed = false;

        // 要素が偶数である限り、 2 で割って次へ
        // 基数のものがあったら中断する
        for (int i = 0; i < n; i++) {
            if (a[i] % 2 == 1) {
                // 偶数じゃないものがあった
                failed = true;
                break;
            } else {
                // 偶数だったので 2 で割る
                a[i] = a[i] / 2;
            }
        }

        // 成否判定
        if (failed) {
            // 基数のものがあったので、ループを抜けてこれまでのカウントを表示
            break;
        } else {
            // すべて 2 で割れた
            count++;
        }
    }

    cout << count << endl;

    return 0;
}

