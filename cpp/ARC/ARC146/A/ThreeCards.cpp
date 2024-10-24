#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main(int argc, char* argv[]) {

    // カードの数 N を取得
    unsigned int n;
    cin >> n;

    // カードを取得
    vector<int> cards(n);
    for (int i = 0; i < n; i++) {
        int num;
        cin >> num;
        cards.push_back(num);
    }

    // 降順でソート
    sort(cards.begin(), cards.end(), std::greater<int>());

    // 大きい順で vector に格納されるので、
    // 先頭 3 つの要素の全組み合わせを試す。
    // ※ ["987", "98", "9"] とかの場合、 "9" を先頭にする必要がある
    string card0 = to_string(cards[0]);
    string card1 = to_string(cards[1]);
    string card2 = to_string(cards[2]);

    vector<string> candidate(6);
    candidate.push_back(card0 + card1 + card2);
    candidate.push_back(card0 + card2 + card1);
    candidate.push_back(card1 + card0 + card2);
    candidate.push_back(card1 + card2 + card0);
    candidate.push_back(card2 + card0 + card1);
    candidate.push_back(card2 + card1 + card0);

    // 試した組み合わせの中から最大のものを表示
    cout << *(max_element(candidate.begin(), candidate.end())) << endl;

    return 0;
}

