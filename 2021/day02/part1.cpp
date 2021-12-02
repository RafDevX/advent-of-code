#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>

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
		string line;
		int horiz = 0;
		int depth = 0;
		while (getline(file, line)) {
			smatch match;
			if (regex_search(line, match, regex("forward (\\d+)"))) {
				int n = atoi(match.str(1).c_str());
				horiz += n;
			} else if (regex_search(line, match, regex("down (\\d+)"))) {
				int n = atoi(match.str(1).c_str());
				depth += n;
			} else if (regex_search(line, match, regex("up (\\d+)"))) {
				int n = atoi(match.str(1).c_str());
				depth -= n;
			}
		}
		cout << "Horiz: " << horiz << "; Depth: " << depth << endl;
		cout << "Horiz * Depth: " << horiz * depth << endl;
	}
	return 0;
}