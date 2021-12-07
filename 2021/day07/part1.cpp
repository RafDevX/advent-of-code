#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int calculate_fuel_cost(vector<int> crabs, int alignAt)
{
	int cost = 0;
	for (int i = 0; i < crabs.size(); i++) {
		cost += abs(crabs[i] - alignAt);
	}
	return cost;
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
		vector<int> crabs;
		int minPos, maxPos;
		bool first = true;
		for (int i; file >> i;) {
			crabs.push_back(i);
			if (first || i < minPos) {
				minPos = i;
			}
			if (first || i > maxPos) {
				maxPos = i;
			}
			first = false;
			if (file.peek() == ',') {
				file.ignore();
			}
		}
		cout << "Brute-forcing in [" << minPos << ", " << maxPos << "]\n";
		first = true;
		int minFuel, alignmentPos;
		for (int pos = minPos; pos <= maxPos; pos++) {
			int fuel = calculate_fuel_cost(crabs, pos);
			if (first || fuel < minFuel) {
				minFuel = fuel;
				alignmentPos = pos;
			}
			first = false;
		}
		cout << "Minimum fuel spent is " << minFuel << " when aligning at position " << alignmentPos << "\n";
	}
	return 0;
}