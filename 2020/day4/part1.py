import os
import re

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()] + [""]

required = ('byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid')
def includes_everything_required(passport):
	for field in required:
		if field not in passport.keys():
			return False
	return True

valid = 0
cur_passport = {}
for line in input_lines:
	if line == "":
		valid += includes_everything_required(cur_passport)
		cur_passport = {}
	else:
		for match in re.finditer(r'(.+?):(.+?)\s+', line + ' '):
			cur_passport[match.group(1)] = match.group(2)

print(f'Found {valid} valid (wink wink) passports')

