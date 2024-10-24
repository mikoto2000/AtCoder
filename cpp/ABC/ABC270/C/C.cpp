#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

bool get_path_internal(vector<vector<int>> *nodes, int y, int current_node, int previous_node, vector<int> *path) {

    // ゴール判定
    if (current_node == y) {
        path->push_back(current_node);
        return true;
    }

    //cout << "current_node: " << current_node << endl;

    //cout << "x: " << x << ", y: " << y << endl;

    for (int next_node : nodes->at(current_node)) {
        // 戻るのは駄目
        if (next_node == previous_node) {
            continue;
        }

        // 次のノードを走査し、true が返ってきてたら(ゴールしてたら) path に
        // 現在のノードを積んでリターン
        if (get_path_internal(nodes, y, next_node, current_node, path)) {
            path->push_back(current_node);
            return true;
        }
    }

    // ゴールできなかった
    return false;
}

vector<int>* get_path(vector<vector<int>> *nodes, int x, int y) {

    // 初期値を設定して深さ優先探索
    vector<int> *path = new vector<int>();
    get_path_internal(nodes, y, x, -1, path);

    return path;
}


int main(int argc, char* argv[]) {

    // 頂点の数 N を取得
    int n;
    cin >> n;

    // X, Y を取得
    int x, y;
    cin >> x >> y;

    vector<vector<int>> nodes(n + 1);

    // ノードに対して接するノードを記録する多次元 vector を作成
    // nodes[対象ノード] = <隣接ノードのリスト>
    for (int i = 0; i < n - 1; i++) {
        int from, to;
        cin >> from >> to;

        nodes[from].push_back(to);
        nodes[to].push_back(from);
    }

    // 単純パスを取得
    auto path = get_path(&nodes, x, y);

    // 標準出力
    for (int i = path->size() - 1; i > 0; i--) {
        cout << path->at(i) << " ";
    }
    cout << path->at(0) << endl;

    return 0;
}

