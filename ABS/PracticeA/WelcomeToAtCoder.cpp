#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 整数 a, b, c を取得
    unsigned short a, b, c;
    cin >> a >> b >> c;

    // 文字列 s を取得
    string s;
    cin >> s;

    // 結果
    cout << (a + b + c) << " " << s << endl;

    return 0;
}

