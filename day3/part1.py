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

trees = 0
line = 0
col = 0
while True:
	if map[line][col % max_cols]:
		trees += 1
	col += 3
	line += 1
	if line == max_lines:
		break

print(f'Found {trees} trees')