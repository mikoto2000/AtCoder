#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // マス s1, s2, s3 を 3 文字の文字列として取得
    string s;
    cin >> s;

    // ビー玉を置く数を数える
    unsigned short count = 0;
    for (int i = 0; i < 3; i++) {
        if (s[i] == '1') {
            // マスに '1' が書いてあったらビー玉を置く
            count++;
        }
    }

    // ビー玉が置かれるマスの数を出力
    cout << count << endl;

    return 0;
}

