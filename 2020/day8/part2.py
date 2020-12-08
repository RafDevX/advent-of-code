import os
import re
import copy

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()]

instructions = []
for line in input_lines:
	match = re.match(r'^(\w+) ([+\-])(\d+)$', line)
	if match:
		instructions += [[match.group(1), int(match.group(2) + match.group(3))]]

def run_boot_code(code): # returns acc value OR None if infinite loop
	acc = 0
	pc = 0 # program counter (cur instruction)
	executed = set()
	while True:
		if pc in executed:
			return None
		elif pc >= len(code):
			return acc
		executed.add(pc)
		(op, arg) = code[pc]
		if op == 'jmp':
			pc += arg
			continue
		elif op == 'acc':
			acc += arg
		pc += 1

for ins_id in range(len(instructions)):
	op = instructions[ins_id][0]
	if op == 'acc':
		continue
	new = copy.deepcopy(instructions)
	new_op = 'nop' if op == 'jmp' else 'jmp'
	new[ins_id][0] = new_op
	result = run_boot_code(new)
	if result is not None:
		print(f'Fixed by switching instruction #{ins_id} from {op} to {new_op}!')
		print('New accumulator value:', result)