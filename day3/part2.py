import os

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()]

map = ()
for i1 in range(len(input_lines)):
	l1 = input_lines[i1]
	line = ()
	for i2 in range(len(l1)):
		line = line + (l1[i2] == '#',)
	map = map + (line,)

max_lines = len(map)
max_cols = len(map[0]) if map else 0

def trees_for_slope(right, down):
	trees = 0
	line = 0
	col = 0
	while True:
		if map[line][col % max_cols]:
			trees += 1
		col += right
		line += down
		if line >= max_lines:
			break
	return trees

slopes_to_check = (
	(1, 1),
	(3, 1),
	(5, 1),
	(7, 1),
	(1, 2),
)
product = 1

for slope in slopes_to_check:
	found = trees_for_slope(*slope)
	product *= found
	print(f'Found {found} trees for slope {slope}')

print('Total product:', product)