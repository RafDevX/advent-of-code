import os
import re

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()] + [""]

def valid_height(h):
	num, unit = h[:-2], h[-2:]
	if not (num.isnumeric() and unit in ('cm', 'in')):
		return False
	
	return 150 <= int(num) <= 193 if unit == 'cm' else 59 <= int(num) <= 76

required = {
	'byr': lambda x: x.isnumeric() and 1920 <= int(x) <= 2002,
	'iyr': lambda x: x.isnumeric() and 2010 <= int(x) <= 2020,
	'eyr': lambda x: x.isnumeric() and 2020 <= int(x) <= 2030,
	'hgt': valid_height,
	'hcl': lambda x: bool(re.match(r'^#[0-9a-f]{6}$', x)),
	'ecl': lambda x: x in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'),
	'pid': lambda x: x.isnumeric() and len(x) == 9
}
def is_valid_passport(passport):
	for field in required.keys():
		if field not in passport.keys():
			return False
		elif not required[field](passport[field]):
			return False
	return True

valid = 0
cur_passport = {}
for line in input_lines:
	if line == "":
		valid += is_valid_passport(cur_passport)
		cur_passport = {}
	else:
		for match in re.finditer(r'(.+?):(.+?)\s+', line + ' '):
			cur_passport[match.group(1)] = match.group(2)

print(f'Found {valid} valid (wink wink) passports')

