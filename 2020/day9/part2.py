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
	for num in inp[preamble_size:]:
		if not valid_number(preamble, num):
			return num
		preamble.pop(0)
		preamble.append(num)

bad = first_malformed_number(input_lines)
contig = []
for i in range(len(input_lines)):
	num = input_lines[i]
	if num == bad:
		continue
	contig = [num]
	j = i
	while sum(contig) <= bad:
		j += 1
		contig.append(input_lines[j])
		if sum(contig) == bad:
			break
	else:
		continue
	break

print('Contigous set:', contig)
print('Encryption weakness:', min(contig) + max(contig))