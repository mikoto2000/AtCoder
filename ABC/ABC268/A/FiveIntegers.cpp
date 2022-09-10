#include <iostream>
#include <unordered_set>

using namespace std;

int main(int argc, char* argv[]) {

    // 標準入力を取得
    unordered_set<short> ns(5);
    for (int i = 0; i < 5; i++) {
        short n;
        cin >> n;
        ns.insert(n);
    }

    // 標準出力
    cout << ns.size() << endl;

    return 0;
}

