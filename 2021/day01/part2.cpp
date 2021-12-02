#include <fstream>
#include <iostream>
#include <queue>
#include <string>

using namespace std;

int sum(deque<int> q)
{
	int sum = 0;
	for (int i = 0; i < q.size(); i++) {
		// cout << ">>" << q[i] << endl;
		sum += q[i];
	}
	// cout << sum << endl;
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
		deque<int> prev_window;
		deque<int> cur_window;
		int c = 0;
		while (getline(file, line)) {
			int n = atoi(line.c_str());
			cur_window.push_back(n);
			if (cur_window.size() > 3) {
				cur_window.pop_front();
			}
			if (prev_window.size() == 3) {
				if (sum(cur_window) > sum(prev_window)) {
					c++;
				}
			}
			prev_window.push_back(n);
			if (prev_window.size() > 3) {
				prev_window.pop_front();
			}
		}
		cout << "# of increases by sliding window of size 3: " << c << endl;
	}
	return 0;
}