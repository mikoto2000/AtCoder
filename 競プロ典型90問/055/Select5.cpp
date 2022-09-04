#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 整数の数 N, 割る数 P, 余り Q を取得
    unsigned short n;
    unsigned int p, q;
    cin >> n >> p >> q;

    // 整数を配列へ格納
    unsigned long long a[n];
    for (int i = 0; i < n; i++) {
        cin >> a[i];
    }

    // 全探索
    unsigned long long count = 0;
    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            for (int k = j + 1; k < n; k++) {
                for (int l = k + 1; l < n; l++) {
                    for (int m = l + 1; m < n; m++) {
                        // 全要素かけ合わせるとオーバーフローするため、
                        // 各要素ごとに余りを求めてからかけ合わせる。
                        if (a[i] % p
                                * a[j] % p
                                * a[k] % p
                                * a[l] % p
                                * a[m] % p
                                == q) {
                            count++;
                        }
                    }
                }
            }
        }
    }

    // 結果出力
    cout << count << endl;

    return 0;
}

