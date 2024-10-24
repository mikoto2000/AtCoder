#include <cstring>
#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    string str;
    cin >> str;
    auto cstr = str.c_str();

    long long str_size = str.size();
    long long current_index = 0;

    bool no_match = false;
    while (current_index < str_size) {
        char buf[8] = {0, 0, 0, 0, 0, 0, 0, 0};

        // 5 文字取得
        memcpy(buf, &(cstr[current_index]), 5);
        current_index += 5;

        // dream/erase 判定
        if (strcmp(buf, "dream") == 0) {
            // dream 系だった

            // er 判定
            char erbuf[3] = {0, 0, 0};

            // 2 文字取得
            memcpy(erbuf, &(cstr[current_index]), 2);
            if (strcmp(erbuf, "er") == 0) {
                // dreamer 確定
                current_index += 2;
            } else {
                // dream 確定
                // do nothing
            }
        } else if (strcmp(buf, "erase") == 0) {
            // erase 系だった

            // er 判定
            char erbuf[2] = {0, 0};

            // 1 文字取得
            memcpy(erbuf, &(cstr[current_index]), 1);
            if (strcmp(erbuf, "r") == 0) {
                // eraser 確定
                current_index += 1;
            } else {
                // erase 確定
                // do nothing
            }
        } else {
            // er -> erase 判定
            if (memcmp(buf, "ase", 3) == 0) {
                // dreamer 判定だったのを dream 判定にして、今回が erase/eraser ということにできる
                current_index -= 7;
            } else {
                no_match = true;
                break;
            }
        }
    }

    // 標準出力
    if (no_match) {
        cout << "NO" << endl;
    } else {
        cout << "YES" << endl;
    }

    return 0;
}

