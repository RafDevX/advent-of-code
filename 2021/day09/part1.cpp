#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

bool is_low_point(vector<vector<int>> heightMap, int i, int j)
{
	bool top = i == 0 || heightMap[i][j] < heightMap[i - 1][j];
	bool bot = i == (heightMap.size() - 1) || heightMap[i][j] < heightMap[i + 1][j];
	bool lft = j == 0 || heightMap[i][j] < heightMap[i][j - 1];
	bool rgt = j == heightMap[0].size() - 1 || heightMap[i][j] < heightMap[i][j + 1];
	return top && bot && lft && rgt;
}

int main(int argc, char *argv[])
{
	if (argc < 2) {
		cout << "Run with text file as argument" << endl;
		return 1;
	}
	fstream file;
	file.open(argv[1], ios::in);
	if (file.is_open()) {
		string line;
		vector<vector<int>> heightMap;
		while (getline(file, line)) {
			vector<int> row;
			for (char c : line) {
				row.push_back(c - '0');
			}
			heightMap.push_back(row);
		}
		int sum = 0;
		for (int i = 0; i < heightMap.size(); i++) {
			for (int j = 0; j < heightMap[i].size(); j++) {
				if (is_low_point(heightMap, i, j)) {
					sum += heightMap[i][j] + 1;
				}
			}
		}
		cout << "Sum: " << sum << "\n";
	}
	return 0;
}