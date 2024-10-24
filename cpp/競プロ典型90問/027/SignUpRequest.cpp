#include <iostream>
#include <set>

using namespace std;

int main(int argc, char* argv[]) {

    // 標準入力を取得
    unsigned int n;
    cin >> n;

    set<string> users;
    for (int i = 0; i < n; i++) {
        string user;
        cin >> user;

        // 既存ユーザーがいるか確認し、いないなら登録
        if (users.find(user) == users.end()) {
            users.insert(user);
            cout << i + 1 << endl;
        }
    }

    return 0;
}

