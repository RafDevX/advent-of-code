import os
import re

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()]

valid = 0
for line in input_lines:
	[minPermitted, maxPermitted, char, pwd] = re.search(r'^(\d+)-(\d+) (\w): (.+)$', line).groups()
	minPermitted, maxPermitted = int(minPermitted), int(maxPermitted)

	if minPermitted <= pwd.count(char) <= maxPermitted:
		valid += 1

print(valid, 'passwords found')