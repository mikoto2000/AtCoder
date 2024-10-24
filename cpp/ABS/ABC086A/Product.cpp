#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 整数 a, b を取得
    unsigned short a, b;
    cin >> a >> b;

    // a と b の積を計算
    unsigned int result = a * b;

    // 偶数判定
    if (result % 2 == 0) {
        // 偶数
        cout << "Even" << endl;
    } else {
        // 基数
        cout << "Odd" << endl;
    }

    return 0;
}

