#include <algorithm>
#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

// from https://stackoverflow.com/a/19483741
vector<char> intersection(vector<char> v1, vector<char> v2)
{
	vector<char> v3;

	sort(v1.begin(), v1.end());
	sort(v2.begin(), v2.end());

	set_intersection(v1.begin(), v1.end(), v2.begin(), v2.end(), back_inserter(v3));

	return v3;
}

/*
	Cannonical representation:
	 aaaa
	b    c
	b    c
	 dddd
	e    f
	e    f
	 gggg
*/
int get_digit_from_cannonical_representation(string word)
{
	map<string, int> digits = {
		{"abcefg", 0},
		{"cf", 1},
		{"acdeg", 2},
		{"acdfg", 3},
		{"bcdf", 4},
		{"abdfg", 5},
		{"abdefg", 6},
		{"acf", 7},
		{"abcdefg", 8},
		{"abcdfg", 9},
	};
	sort(word.begin(), word.end());
	return digits[word];
}

string convert_word(string oldWord, map<char, char> conversion)
{
	string newWord = "";
	for (char c : oldWord) {
		newWord.push_back(conversion[c]);
	}
	return newWord;
}

int get_output_value(vector<vector<string>> entry)
{
	map<char, char> conversion; // conversion[display] = cannonical
	map<char, vector<char>> possible;
	map<char, int> counts = {{'a', 0}, {'b', 0}, {'c', 0}, {'d', 0}, {'e', 0}, {'f', 0}, {'g', 0}};
	for (string word : entry[0]) {
		for (char c : word) {
			counts[c]++;
		}
	}
	for (char c : "abcdefg") {
		switch (counts[c]) {
		case 8:
			possible[c] = {'a', 'c'};
			break;
		case 6:
			possible[c] = {'b'};
			break;
		case 7:
			possible[c] = {'d', 'g'};
			break;
		case 4:
			possible[c] = {'e'};
			break;
		case 9:
			possible[c] = {'f'};
			break;
		}
	}

	for (string word : entry[0]) {
		vector<char> restriction;
		if (word.size() == 2) {
			restriction = {'c', 'f'};
		} else if (word.size() == 4) {
			restriction = {'b', 'c', 'd', 'f'};
		} else if (word.size() == 3) {
			restriction = {'a', 'c', 'f'};
		}

		if (restriction.size() > 0) {
			for (char c : word) {
				possible[c] = intersection(possible[c], restriction);
			}
		}
	}

	for (char c : "abcdefg") {
		if (possible[c].size() == 1) {
			for (char d : "abcdefg") {
				if (c != d) {
					/*cout << "Prev: <";
					for (char i : possible[d])
						cout << i << ",";
					cout << ">\n";*/
					possible[d].erase(remove(possible[d].begin(), possible[d].end(), c), possible[d].end());
					/*cout << "Now: <";
					for (char i : possible[d])
						cout << i << ",";
					cout << ">\n";*/
				}
			}
		}
	}

	for (char c : "abcdefg") {
		if (possible[c].size() != 1) {
			cout << "Could not narrow down from <";
			for (char d : possible[c]) {
				cout << d << ",";
			}
			cout << "> for char " << c << " in entry with head [";
			for (string word : entry[0]) {
				cout << word << " ";
			}
			cout << "]\n";
			exit(1);
		}
		conversion[c] = possible[c][0];
	}

	int val = 0;
	for (string word : entry[1]) {
		string cannon = convert_word(word, conversion);
		val = val * 10 + get_digit_from_cannonical_representation(cannon);
	}
	return val;
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
			c += get_output_value(entry);
		}
		cout << "Sum: " << c << "\n";
	}
	return 0;
}