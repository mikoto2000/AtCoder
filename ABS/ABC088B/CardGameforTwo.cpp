#include <algorithm>
#include <iostream>
#include <list>

using namespace std;

int main(int argc, char* argv[]) {

    // カード枚数 N を取得
    unsigned short n;
    cin >> n;

    // カードを取得
    list<unsigned short> cards;
    for (int i = 0; i < n; i++) {
        unsigned short card;
        cin >> card;

        cards.push_back(card);
    }

    // ゲーム実施
    unsigned int alice = 0, bob = 0;
    for (int i = 0; i < n; i++) {
        // 最大の値を取得してリストから削除
        auto max = max_element(cards.begin(), cards.end());
        cards.erase(max);

        // 手番に応じた相手に取得したカードを追加
        if (i % 2 == 0) {
            // alice の手番
            alice += *max;
        } else {
            // bob の手番
            bob += *max;
        }
    }

    // 結果出力
    cout << alice - bob << endl;

    return 0;
}

