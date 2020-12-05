import os

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()]

def get_partition(s, max_val, lower_half_char):
	considering = tuple(range(max_val))
	while len(s):
		half = len(considering) // 2
		considering = considering[:half] if s[0] == lower_half_char else considering[half:]
		s = s[1:]
	return considering[0]

def get_row(s):
	return get_partition(s, 128, 'F')

def get_col(s):
	return get_partition(s, 8, 'L')

def get_seat_coordinates(seat):
	return (get_row(seat[:7]), get_col(seat[7:]))

def get_seat_id(seat):
	(row, col) = get_seat_coordinates(seat)
	return row * 8 + col

seats = ()
highest_id = 0
for line in input_lines:
	seat_id = get_seat_id(line)
	if seat_id > highest_id:
		highest_id = seat_id
	seats += (seat_id,)

for seat_id in range(1, highest_id):
	if seat_id not in seats and seat_id - 1 in seats and seat_id + 1 in seats:
		print('My seat is', seat_id)
		break