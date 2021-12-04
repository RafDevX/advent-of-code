#include <algorithm>
#include <bitset>
#include <cmath>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

bool most_common_msb(vector<int> nums, unsigned int binLen)
{
	int zeroCount = 0;
	int sz = nums.size();

	for (unsigned int i = 0; i < sz; i++) {
		if ((nums[i] & 1 << (binLen - 1)) == 0) {
			zeroCount++;
		}
	}

	return zeroCount <= sz / 2;
}

int most_common_recursive_filter(vector<int> nums, unsigned int binLen, bool leastCommon = false)
{
	bool most = most_common_msb(nums, binLen);
	vector<int> filtered;
	copy_if(nums.begin(), nums.end(), back_inserter(filtered), [=](int i) {
		bool val = min((int)(i & 1 << (binLen - 1)), 1) == most;
		return leastCommon ? !val : val;
	});
	if (filtered.size() <= 1) {
		return filtered[0];
	} else {
		return most_common_recursive_filter(filtered, binLen - 1, leastCommon);
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
		vector<int> nums;
		unsigned int binLen = 0;
		while (getline(file, line)) {
			binLen = line.size();
			nums.push_back(stoi(line, nullptr, 2));
		}
		int og_rating = most_common_recursive_filter(nums, binLen);
		int cs_rating = most_common_recursive_filter(nums, binLen, true);
		cout << "Oxygen Generator Rating: " << og_rating << endl;
		cout << "CO2 Scrubber Rating: " << cs_rating << endl;
		cout << "Power Consumption: " << og_rating * cs_rating << endl;
	}
	return 0;
}