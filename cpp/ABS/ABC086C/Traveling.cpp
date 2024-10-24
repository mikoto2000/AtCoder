#include <cmath>
#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // 訪問先情報数 N を取得
    unsigned int n;
    cin >> n;

    bool is_ok = true;

    // ひとつ前の訪問先
    int tp = 0, xp = 0, yp = 0;

    for (int i = 0; i < n; i++) {
        // 今回注目する訪問先
        int t, x, y;
        cin >> t >> x >> y;

        // 前回訪問先から今回訪問先までの道のりを計算
        int distance = abs(xp - x) + abs(yp - y);

        // 移動時間を計算
        int time = t - tp;

        // 距離的に移動可能か判定
        if (distance > time) {
            // 距離的に移動不可能
            is_ok = false;
            break;
        }

        // タイミング的に移動可能か判定
        // 時間調整のために反復横跳びする必要があるため、
        // 到着後の残り時間が偶数でないと不可能。
        if ((time - distance) % 2 == 1) {
            // タイミング的に移動不可能
            is_ok = false;
            break;
        }

        // ひとつ前の情報を更新して次のループへ
        tp = t;
        xp = x;
        yp = y;
    }

    // 結果出力
    if (is_ok) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }


    return 0;
}

