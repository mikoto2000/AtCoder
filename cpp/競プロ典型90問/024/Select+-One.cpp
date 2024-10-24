#include <cmath>
#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 数列の長さ N と操作数 K を取得
    unsigned short n;
    unsigned int k;
    cin >> n >> k;

    // 数列 A を取得
    int a[n];
    for (int i = 0; i < n; i++) {
        cin >> a[i];
    }

    // 数列 B を取得
    int b[n];
    for (int i = 0; i < n; i++) {
        cin >> b[i];
    }

    // 「各要素の差」が、「A を B と等しくするための最小操作数」
    long sum_diff = 0;
    for (int i = 0; i < n; i++) {
        sum_diff += abs(a[i] - b[i]);
    }

    // A と B が等しくなったら、
    // 反復横跳びで時間調整をするしかない。
    // つまり、 K から「A を B と等しくするための最小操作数」を引いた数が偶数なら OK。
    // また、K から「A を B と等しくするための最小操作数」を引いた数が負の数だとそもそも手数が足りないという事

    if (k - sum_diff < 0) {
        cout << "No" << endl;
        return 0;
    }

    if ((k - sum_diff) % 2 == 0) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }

    return 0;
}

