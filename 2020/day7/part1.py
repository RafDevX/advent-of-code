import os
import re

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()]

parsed = {}
for line in input_lines:
	match = re.match(r'(\w+ \w+) bags contain (.+)\.', line)
	if match:
		color, contained = match.group(1), match.group(2)
		color_contains = {}
		for m in re.finditer(r'(\d+) (\w+ \w+) bags?', contained):
			qty, contained_color = m.group(1), m.group(2)
			if qty and contained_color and qty.isnumeric():
				qty = int(qty)
				color_contains[contained_color] = qty
		parsed[color] = color_contains

def what_colors_contain(color):
	result = set()
	for container in parsed.keys():
		if color in parsed[container].keys():
			result.add(container)
			result = result.union(what_colors_contain(container))
	return result

shiny_gold = what_colors_contain('shiny gold')
print(f'There are {len(shiny_gold)} colors of bags that contain shiny gold bags')