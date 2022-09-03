#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 整数列の長さ N, M を取得
    unsigned int n, m;
    cin >> n >> m;

    // 数列 A を取得
    long long a[n];
    for (int i = 0; i < n; i++) {
        cin >> a[i];
    }

    /**
     * 計算した部分文字列を S とし、次の部分文字列を NS とする。
     * NS を計算するための sum(A[i]..A[i+m]) を T とする。
     *
     * See: https://www.youtube.com/watch?v=yiFC8tQG71s
     * See: https://twitter.com/mikoto2000/status/1566074933129920513
     */

    // S の計算
    long long s = 0;
    for (int i = 0; i < m; i++) {
        s += a[i] * (i + 1);
    }

    // T の計算
    long long t = 0;
    for (int i = 0; i < m; i++) {
        t += -a[i];
    }

    // 最初に計算した S, T を基に、2 番目以降の S を計算していく
    // 計算しながら max の更新も行う。
    long long max = s;
    for (int i = 1; i <= n - m; i++) {
        // 次の S を計算
        s += t + a[i + (m - 1)] * m;

        // 次の T を計算
        // 先頭を押し出して末尾を追加
        t += a[i - 1] - a[i + (m - 1)];

        // 最大値更新
        if (s > max) {
            max = s;
        }
    }

    // 標準出力
    cout << max << endl;

    return 0;
}

