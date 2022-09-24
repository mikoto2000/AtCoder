#include <cmath>
#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    // X, Y, Z を取得
    int x, y, z;
    cin >> x >> y >> z;

    if (x == 0) {
        cout << 0 << endl;
        return 0;
    } else if (x > 0) {
        if (y < 0) {
            cout << x << endl;
            return 0;
        } else if (y < x) {
            // x と原点の間に壁がある
            if (z > y) {
                // ハンマーが壁のむこうにある
                cout << -1 << endl;
                return 0;
            } else {
                // ハンマーが壁のこちらにある
                cout <<
                    abs(z) // 原点からハンマーまでの距離
                    + x - z // ゴールからハンマーまでの距離
                    << endl;
            }
        } else {
            // x と原点の間に壁がない
            cout << x << endl;
            return 0;
        }
    } else if (x < 0) {
        if (y > 0) {
            cout << -x << endl;
            return 0;
        } else if (y > x) {
            // x と原点の間に壁がある
            if (z < y) {
                // ハンマーが壁のむこうにある
                cout << -1 << endl;
                return 0;
            } else {
                // ハンマーが壁のこちらにある
                cout <<
                    abs(z) // 原点からハンマーまでの距離
                    + z - x // ゴールからハンマーまでの距離
                    << endl;
            }
        } else {
            // x と原点の間に壁がない
            cout << -x << endl;
            return 0;
        }
    }

    return 0;
}

