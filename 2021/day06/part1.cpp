#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

void show_school(vector<int> school, string prefix)
{
	cout << prefix;
	for (int fish : school) {
		cout << fish << ",";
	}
	cout << "\n";
}

void step(vector<int> *school)
{
	int sz = (*school).size(); // calculated at the beginning so it doesn't count new fish
	for (int i = 0; i < sz; i++) {
		if ((*school)[i] == 0) {
			(*school)[i] = 6;
			(*school).push_back(8);
		} else {
			(*school)[i]--;
		}
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
		vector<int> school;
		for (int i; file >> i;) {
			school.push_back(i);
			if (file.peek() == ',') {
				file.ignore();
			}
		}
		show_school(school, "Initial Fish: ");
		for (int i = 1; i <= 80; i++) {
			step(&school);
			//show_school(school, "Fish after " + (string)(i < 10 ? " " : "") + to_string(i) + " day" + (i == 1 ? ":  " : "s: "));
		}
		cout << "Total " << school.size() << " fish";
	}
	return 0;
}