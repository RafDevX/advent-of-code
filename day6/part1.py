import os

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()]

current_group = tuple(range(ord('z') - ord('a') + 1))
clear_before_counting = ()
total_sum = 0
for line in input_lines + [""]:
	if line == "":
		total_sum += len(current_group)
		current_group = tuple(range(ord('z') - ord('a') + 1))
	else:
		new = ()
		for q in current_group:
			if (chr(q + ord('a'))) in line:
				new += (q,)
		current_group = new

print("Total sum:", total_sum)