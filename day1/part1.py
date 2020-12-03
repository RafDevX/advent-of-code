import os

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [int(x.strip()) for x in input_file.readlines()]

for i1 in range(len(input_lines)):
	n1 = input_lines[i1]
	for i2 in range(i1 + 1, len(input_lines)):
		n2 = input_lines[i2]
		if n1 + n2 == 2020:
			print(f"Found: n1 = {n1}, n2 = {n2}, n1 * n2 = {n1 * n2}")
			break