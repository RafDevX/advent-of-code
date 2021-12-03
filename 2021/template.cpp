#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

// yes, I hate this too, and I know there are much better ways

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
		// start operations
		while (getline(file, line)) {
			// per line operations
		}
		// end operations
	}
	return 0;
}