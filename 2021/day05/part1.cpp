#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

void mark_line_in_map(vector<vector<int>> line, vector<vector<int>> *map)
{
	if (line[0][0] == line[1][0]) {
		for (int i = min(line[0][1], line[1][1]); i <= max(line[0][1], line[1][1]); i++) {
			cout << "v-point (" << line[0][0] << ", " << i << ")\n";
			(*map)[line[0][0]][i]++;
		}
		//exit(1);
	} else if (line[0][1] == line[1][1]) {
		for (int i = min(line[0][0], line[1][0]); i <= max(line[0][0], line[1][0]); i++) {
			cout << "h-point (" << i << ", " << line[0][1] << ")\n";
			(*map)[i][line[0][1]]++;
		}
	} else {
		//cerr << "Diagonal line (" << line[0][0] << ", " << line[0][1] << ") -> (" << line[1][0] << ", " << line[1][1] << ")!\n";
		//exit(1);
	}
}

void show_map(vector<vector<int>> map)
{
	for (int i = 0; i < map.size(); i++) {
		for (int j = 0; j < map[i].size(); j++) {
			cout << map[i][j];
		}
		cout << "\n";
	}
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
		int maxX = 0;
		int maxY = 0;
		vector<vector<vector<int>>> lines;
		while (getline(file, line)) {
			smatch match;
			if (regex_search(line, match, regex("(\\d+),(\\d+) -> (\\d+),(\\d+)"))) {
				int x1 = atoi(match.str(1).c_str());
				int y1 = atoi(match.str(2).c_str());
				int x2 = atoi(match.str(3).c_str());
				int y2 = atoi(match.str(4).c_str());
				maxX = max(maxX, max(x1, x2));
				maxY = max(maxY, max(y1, y2));
				vector<vector<int>> line;
				vector<int> from = {x1, y1};
				vector<int> to = {x2, y2};
				line.push_back(from);
				line.push_back(to);
				lines.push_back(line);
			}
		}
		vector<vector<int>> map = vector(maxY + 1, vector(maxX + 1, 0));
		for (vector<vector<int>> line : lines) {
			mark_line_in_map(line, &map);
		}
		int c = 0;
		for (int i = 0; i < map.size(); i++) {
			for (int j = 0; j < map[i].size(); j++) {
				if (map[i][j] >= 2) {
					c++;
				}
			}
		}
		show_map(map);
		cout << "At least 2 lines overlap in [" << c << "] points\n";
	}
	return 0;
}