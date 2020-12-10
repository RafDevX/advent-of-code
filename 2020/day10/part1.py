import os

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [int(x.strip()) for x in input_file.readlines()]

input_lines.sort()

inp = 0
out = input_lines[-1] + 3
diffs = ()

while inp < out - 3:
	adapter = input_lines.pop(0)
	diff = adapter - inp
	if not (1 <= diff <= 3):
		continue
	diffs += (diff,)
	inp = adapter

c1, c3 = diffs.count(1), diffs.count(3) + 1
print(f'Distribution: 1-jolt - {c1}; 3-jolts - {c3}; answer: {c1 * c3}')