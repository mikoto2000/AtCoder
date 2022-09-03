#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 現在の曜日 S を取得
    string s;
    cin >> s;

    unsigned short number = 0;
    if (s == "Monday") {
        number = 1;
    } else if (s == "Tuesday") {
        number = 2;
    } else if (s == "Wednesday") {
        number = 3;
    } else if (s == "Thursday") {
        number = 4;
    } else if (s == "Friday") {
        number = 5;
    } else {
        number = 0;
    }

    // 標準出力
    cout << 6 - number << endl;

    return 0;
}

