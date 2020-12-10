import os

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input_t.txt', 'r')
input_lines = [int(x.strip()) for x in input_file.readlines()]

input_lines.sort()

inp = 0
out = input_lines[-1] + 3
arrangements = 0

def is_valid_arrangement(adapters_in):
	adapters = adapters_in.copy()
	inp = 0
	out = input_lines[-1] + 3
	while inp < out - 3:
		if not len(adapters):
			return False
		adapter = adapters.pop(0)
		diff = adapter - inp
		if not (1 <= diff <= 3):
			continue
		inp = adapter
	return True

# HOW?

print('Total arrangements:', check_sub_arrangements(input_lines))