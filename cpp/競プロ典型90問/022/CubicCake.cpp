#include <iostream>
#include <numeric>

using namespace std;

int main(int argc, char* argv[]) {

    // 幅 A, 奥行 B, 高さ C を取得
    unsigned long a, b, c;
    cin >> a >> b >> c;

    // 最大公約数を計算
    unsigned long num = gcd(gcd(a, b), c);

    // 各次元の切る数を計算
    unsigned long cut_a = a / num - 1;
    unsigned long cut_b = b / num - 1;
    unsigned long cut_c = c / num - 1;

    // 標準出力
    cout << cut_a + cut_b + cut_c << endl;

    return 0;
}

