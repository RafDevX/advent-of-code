import os
import re

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()]

valid = 0
for line in input_lines:
	[pos1, pos2, char, pwd] = re.search(r'^(\d+)-(\d+) (\w): (.+)$', line).groups()
	pos1, pos2 = int(pos1) - 1, int(pos2) - 1

	if (pwd[pos1] == char) ^ (pwd[pos2] == char):
		valid += 1

print(valid, 'passwords found')