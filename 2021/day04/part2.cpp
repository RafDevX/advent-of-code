#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

bool mark_number_in_card(vector<vector<int>> *card, int num)
{
	for (int row = 0; row < (*card).size(); row++) {
		bool incompleteRow = false;
		int marked = false;
		for (int col = 0; col < (*card)[row].size(); col++) {
			if ((*card)[row][col] == num) {
				(*card)[row][col] = -1;
				marked = true;

				bool incompleteCol = false;
				for (int i = 0; i < (*card).size(); i++) {
					if ((*card)[i][col] >= 0) {
						incompleteCol = true;
						break;
					}
				}
				if (!incompleteCol)
					return true;

			} else if ((*card)[row][col] >= 0) {
				incompleteRow = true;
			}
		}
		if (marked && !incompleteRow)
			return true;
	}
	return false;
}

int sum_unmarked(vector<vector<int>> card)
{
	int sum = 0;
	for (vector<int> row : card) {
		for (int num : row) {
			if (num >= 0) {
				sum += num;
			}
		}
	}
	return sum;
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
		vector<int> nums;
		vector<vector<vector<int>>> cards;
		int row, col;
		bool firstLine = true;
		while (getline(file, line)) {
			stringstream ss(line);
			if (firstLine) {
				for (int i; ss >> i;) {
					nums.push_back(i);
					if (ss.peek() == ',') {
						ss.ignore();
					}
				}
				firstLine = false;
			} else if (line == "") {
				cards.push_back(vector(5, vector(5, 0)));
				row = 0;
				col = 0;
			} else {
				// oh god i hate how wet this whole thing is
				while (ss.peek() == ' ') {
					ss.ignore();
				}
				for (int i; ss >> i;) {
					cards[cards.size() - 1][row][col] = i;
					if (++col >= 5) {
						col = 0;
						row++;
					}
					while (ss.peek() == ' ') {
						ss.ignore();
					}
				}
			}
		}
		for (int num : nums) {
			bool stop = false;
			vector<int> toRemove;
			int removed = 0;
			for (int i = 0; i < cards.size(); i++) {
				if (mark_number_in_card(&cards[i], num)) {
					toRemove.push_back(i);
				}
			}
			for (int pos : toRemove) {
				if (cards.size() > 1) {
					cards.erase(cards.begin() + pos - removed++);
				} else {
					int unmarked = sum_unmarked(cards[0]);
					cout << "Number: " << num << "\n";
					cout << "Unmarked Numbers: " << unmarked << "\n";
					cout << "Score: " << num * unmarked << "\n";
					stop = true;
					break;
				}
			}
			if (stop)
				return 0;
		}
		cout << "No winner found\n";
		// for (auto card : cards) {
		// 	for (auto line : card) {
		// 		for (auto num : line) {
		// 			cout << "|" << num;
		// 		}
		// 		cout << "|\n---\n";
		// 	}
		// 	cout << "\n===\n";
		// }
	}
	return 0;
}