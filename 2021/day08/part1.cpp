#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main(int argc, char *argv[])
{
	if (argc < 2) {
		cout << "Run with text file as argument" << endl;
		return 1;
	}
	fstream file;
	file.open(argv[1], ios::in);
	if (file.is_open()) {
		vector<vector<vector<string>>> entries;
		string line;
		regex rg = regex("(\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) \\| (\\w+) (\\w+) (\\w+) (\\w+)");
		while (getline(file, line)) {
			smatch match;
			if (regex_search(line, match, rg)) {
				vector<string> head, tail;
				for (int i = 1; i <= 14; i++) {
					if (i <= 10) {
						head.push_back(match.str(i));
					} else {
						tail.push_back(match.str(i));
					}
				}
				entries.push_back({head, tail});
			}
		}
		int c = 0;
		for (vector<vector<string>> entry : entries) {
			for (string word : entry[1]) {
				int sz = word.size();
				if (sz == 2 || sz == 4 || sz == 3 || sz == 7) {
					c++;
				}
			}
		}
		cout << "Count: " << c << "\n";
	}
	return 0;
}