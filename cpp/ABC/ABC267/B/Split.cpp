#include <iostream>
#include <regex>

using namespace std;

int main(int argc, char* argv[]) {

    // ピンは位置情報を取得
    string str;
    cin >> str;

    // 配列のインデックス : ピンのインデックス
    // 0 : 6
    // 1 : 3
    // 2 : 1, 7
    // 3 : 0, 4
    // 4 : 2, 8
    // 5 : 5
    // 6 : 9
    int column_pin_map[7][2] = {
        { 6, -1 }, // 0
        { 3, -1 }, // 1
        { 1, 7 }, // 2
        { 0, 4 }, // 3
        { 2, 8 }, // 4
        { 5, -1 }, // 5
        { 9, -1 }, // 6
    };


    // 1 本でもある colum を 1, 1 本もない colum を 0 とした文字列を作成
    string column_status = "";
    for (int i = 0; i < 7; i++) {
        if (str[column_pin_map[i][0]] == '1'
                || (str[column_pin_map[i][1]] == -1 || str[column_pin_map[i][1]] == '1')) {
            column_status += '1';
        } else {
            column_status += '0';
        }
    }

    // ピン 1 の判定
    if (str[0] == '1') {
        // ピン 1 が倒れていないのでスプリットはあり得ない
        cout << "No" << endl;
        return 0;
    }

    // 文字列内に "10+1" が含まれていればスプリット
    regex pattern(".*10+1.*");
    if (regex_match(column_status, pattern)) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }

    return 0;
}

