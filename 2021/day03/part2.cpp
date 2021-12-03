#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int most_common(vector<vector<int>> count, bool least_common = false)
{
	string num = "";
	for (int i = 0; i < count.size(); i++) {
		char n = '0';
		if (count[i][1] > count[i][0]) {
			n = '1';
		}
		if (least_common) {
			n = n == '1' ? '0' : '1';
		}
		num += n;
	}
	return stoi(num, nullptr, 2);
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
		vector<vector<int>> count;
		bool firstLine = true;
		while (getline(file, line)) {
			for (unsigned int i = 0; i < line.length(); i++) {
				int bit = line[i] == '1' ? 1 : 0;
				if (firstLine) {
					count.push_back(vector<int>(2, 0));
				}
				count[i][bit]++;
			}
			firstLine = false;
		}
		int gamma_rate = most_common(count);
		int epsilon_rate = most_common(count, true);
		cout << "Gamma Rate: " << gamma_rate << ", Epsilon Rate: " << epsilon_rate << endl;
		cout << "Power Consumption: " << gamma_rate * epsilon_rate << endl;
	}
	return 0;
}