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

def how_many_bags_contained_in(color):
	total = 0
	contained = parsed[color]
	for contained_color in contained.keys():
		qty = contained[contained_color]
		total += qty * (1 + how_many_bags_contained_in(contained_color))
	return total

shiny_gold = how_many_bags_contained_in('shiny gold')
print(f'A shiny gold bag must contain {shiny_gold} other bags')