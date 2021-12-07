#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

void show_school(vector<long long> school, string prefix)
{
	vector<long long> fs;
	for (long long i = 0; i < school.size(); i++) {
		for (long long j = 0; j < school[i]; j++) {
			fs.push_back(i);
		}
	}
	cout << prefix;
	for (long long fish : fs) {
		cout << fish << ",";
	}
	cout << "\n";
}

void step(vector<long long> *school)
{
	vector<long long> newSchool;
	for (long long i = 0; i < (*school).size(); i++) {
		newSchool.push_back((*school)[i]);
	}
	for (long long i = 0; i < (*school).size(); i++) {
		if (i == 0) {
			newSchool[6] += (*school)[0];
			newSchool[8] += (*school)[0];
			newSchool[0] = 0;
		} else {
			newSchool[i - 1] += (*school)[i];
			newSchool[i] -= (*school)[i];
		}
	}
	for (long long i = 0; i < (*school).size(); i++) {
		(*school)[i] = newSchool[i];
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
		vector<long long> school;
		for (long long i = 0; i <= 8; i++) {
			school.push_back(0);
		}
		for (long long i; file >> i;) {
			school[i]++;
			if (file.peek() == ',') {
				file.ignore();
			}
		}
		show_school(school, "Initial Fish: ");
		for (long long i = 1; i <= 256; i++) {
			step(&school);
			// show_school(school, "Fish after " + (string)(i < 10 ? " " : "") + to_string(i) + " day" + (i == 1 ? ":  " : "s: "));
		}
		long long c = 0;
		for (long long i = 0; i < school.size(); i++) {
			c += school[i];
		}
		cout << "Total " << c << " fish";
	}
	return 0;
}