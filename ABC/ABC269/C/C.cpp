#include <bitset>
#include <iostream>
#include <limits.h>
#include <set>

using namespace std;

unsigned long get_digit(unsigned long n) {
    unsigned long tmp = n;
    unsigned long n_digit = 1;
    while (true) {
        tmp = tmp >> 1;
        if (tmp != 0) {
            n_digit++;
        } else {
            break;
        }
    };

    return n_digit;
}

bool operator<(const bitset<64> &a, const bitset<64> &b) noexcept {
  return a.to_ulong() < b.to_ulong();
}

void get_targets(set<unsigned long> &targets, bitset<64> n, bitset<64> target, unsigned long current_digit) {
    if (current_digit == ULONG_MAX) {
        return;
    }

    if (n.test(current_digit)) {

        // ビットをたてた
        bitset<64> next_enabled_target = target.set(current_digit);
        targets.insert(next_enabled_target.to_ulong());
        get_targets(targets, n, next_enabled_target, current_digit - 1);

        // ビットをたてなかった
        bitset<64> next_disabled_target = target.reset(current_digit);
        targets.insert(next_disabled_target.to_ulong());
        get_targets(targets, n, next_disabled_target, current_digit - 1);

    } else {
        // n がゼロなのでゼロ固定でよい。スキップ
        get_targets(targets, n, target, current_digit - 1);
    }
}

int main(int argc, char* argv[]) {

    // N を取得
    unsigned long n_tmp;
    cin >> n_tmp;

    bitset<64> n(n_tmp);
    unsigned long target_digit = get_digit(n_tmp);

    // N 中の立っているビットを全探索する
    set<unsigned long> targets;
    bitset<64> target;
    target.reset();
    get_targets(targets, n, target, target_digit - 1);

    // ゼロは必ず入るのでここで insert
    targets.insert(0);

    // 結果表示
    for (auto target : targets) {
        cout << target << endl;
    }

    return 0;
}

