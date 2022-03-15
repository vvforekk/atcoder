#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

// input
int H, S, C, a, b;
vector<int> K;
vector<vector<int> > N, A, B;

void read_input() {
    cin >> S >> C >> H >> a >> b;
    N.resize(S);
    for (int s = 0; s < S; s++) {
        N[s].resize(C);
        for (int c = 0; c < C; c++) cin >> N[s][c];
    }
    K.resize(S);
    for (int s = 0; s < S; s++) cin >> K[s];
    A.resize(S);
    for (int s = 0; s < S; s++) {
        A[s].resize(S);
        for (int t = 0; t < S; t++) cin >> A[s][t];
    }
    B.resize(C);
    for (int c = 0; c < C; c++) {
        B[c].resize(C);
        for (int d = 0; d < C; d++) cin >> B[c][d];
    }
}

void write_output(const vector<pair<int, int> > &ans) {
    cout << ans.size() << '\n';
    for (int i = 0; i < int(ans.size()); i++) {
        if (ans[i].first >= 0) {
            cout << ans[i].first + 1 << ' ' << ans[i].second + 1 << '\n';
        } else {
            cout << ans[i].first << '\n';
        }
    }
}

int main() {
    read_input();

    vector<pair<int, int> > ans;
    vector<int> hanger_on_hook(H, -1);
    vector<int> hangers_available = K;
    int pos = 0, prev_s = -1, prev_c = -1;
    for (int s = 0; s < S; s++) {
        for (int c = 0; c < C; c++) { // assign item (s, c)
            if (N[s][c] == 0) continue;

            // need this amount of empty hooks
            const int setup = (prev_s == -1 ? 0 : max(A[prev_s][s], B[prev_c][c]));
            for (int i = 0; i < setup; i++) {
                ans.emplace_back(-2, 0);
                if (pos == H - 1) pos = 0; else pos++;
            }

            // put (s, c)
            for (int n = 0; n < N[s][c]; ) {
                if (hanger_on_hook[pos] != s and hangers_available[s] == 0) {
                    // run out of hangers
                    ans.emplace_back(-2, 0);
                    if (pos == H - 1) pos = 0; else pos++;
                } else {
                    ans.emplace_back(s, c);
                    n++;
                    if (hanger_on_hook[pos] != s) {
                        if (hanger_on_hook[pos] >= 0) {
                            hangers_available[hanger_on_hook[pos]]++;
                        }
                        hangers_available[s]--;
                        hanger_on_hook[pos] = s;
                    }
                    if (pos == H - 1) pos = 0; else pos++;
                }
                prev_s = s;
                prev_c = c;
            }
        }
    }

    // write answer
    write_output(ans);
}
