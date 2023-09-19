#include <iostream>
#include <vector>

using namespace std;


struct Item {
    int height;
    int value;
};


int knapsack(vector<Item>& itens, int Size) {
    int n = itens.size();
    vector<vector<int>> dynprog(n + 1, vector<int>(Size + 1, 0));

    for (int i = 1; i <= n; i++) {
        for (int j = 0; j <= Size; j++) {
            if (itens[i - 1].height <= j) {
                dynprog[i][j] = max(dynprog[i - 1][j], dynprog[i - 1][j - itens[i - 1].height] + itens[i - 1].value);
            } else {
                dynprog[i][j] = dynprog[i - 1][j];
            }
        }
    }

    return dynprog[n][Size];
}

int main() {
    vector<Item> itens = {{3, 2}, {1, 2}, {3, 4}, {4, 5}, {2, 3}};
    int Size = 7;
    int resultado = knapsack(itens, Size);

    cout << "Maior valor possivel: " << resultado << endl;

    return 0;
}

