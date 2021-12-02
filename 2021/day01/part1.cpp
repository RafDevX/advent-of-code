#include <fstream>
#include <iostream>
#include <string>

using namespace std;

int main()
{
	fstream file;
	file.open("input.txt", ios::in);
	if (file.is_open()) {
		string line;
		int last;
		bool first = true;
		int c = 0;
		while (getline(file, line)) {
			int n = atoi(line.c_str());
			if (!first && n > last) {
				c++;
			}
			last = n;
			first = false;
		}
		cout << "# of increases: " << c << endl;
	}
	return 0;
}