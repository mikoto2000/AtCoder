#include <algorithm>
#include <iostream>
#include <set>

using namespace std;

int main(int argc, char* argv[]) {

    // 餅の枚数 N を取得
    unsigned short n;
    cin >> n;

    // 持っている餅の情報を取得
    // 与えられた入力を set に insert することで、
    // set の要素数がそのまま鏡餅の最大段数になる。
    set<unsigned short> mochis;
    for (int i = 0; i < n; i++) {
        unsigned short d;
        cin >> d;
        mochis.insert(d);
    }

    // 標準出力
    cout << mochis.size() << endl;

    return 0;
}

