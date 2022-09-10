#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 文字列 S, T を取得
    string s, t;
    cin >> s >> t;

    

    if (t.find(s) == 0) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }

    return 0;
}

