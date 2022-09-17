#include <iostream>
#include <limits.h>

using namespace std;


int main(int argc, char* argv[]) {

    string strs[10];

    int a = INT_MAX, b = INT_MIN, c = INT_MAX, d = INT_MAX;
    for (int i = 0; i < 10; i++) {
        string str;
        cin >> str;

        int first = str.find("#");
        int last = str.rfind("#");


        if (first != string::npos) {
            a = min(a , i);
            b = max(b , i);

            c = first;
            d = last;
        }

    }

    // 標準出力
    cout << a + 1 << " " << b + 1 << endl;
    cout << c + 1 << " " << d + 1<< endl;

    return 0;
}

