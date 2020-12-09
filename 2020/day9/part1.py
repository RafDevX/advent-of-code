import os

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [int(x.strip()) for x in input_file.readlines()]

def valid_number(preamble, num):
	for v in preamble:
		if (num - v) in preamble:
			return True
	return False

def first_malformed_number(inp, preamble_size = 25):
	preamble = inp[:preamble_size]
	for i in range(preamble_size, len(inp)):
		num = inp[i]
		if not valid_number(preamble, num):
			return num
		preamble.pop(0)
		preamble.append(num)

print('First malformed:', first_malformed_number(input_lines))