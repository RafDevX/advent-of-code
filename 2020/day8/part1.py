import os
import re

path = os.path.dirname(__file__)
if path:
	os.chdir(path)

input_file = open('./input.txt', 'r')
input_lines = [x.strip() for x in input_file.readlines()]

instructions = ()
for line in input_lines:
	match = re.match(r'^(\w+) ([+\-])(\d+)$', line)
	if match:
		instructions += ((match.group(1), int(match.group(2) + match.group(3))),)

acc = 0
pc = 0 # program counter (cur instruction)
executed = set()
while True:
	if pc in executed:
		break
	executed.add(pc)
	(op, arg) = instructions[pc]
	if op == 'jmp':
		pc += arg
		continue
	elif op == 'acc':
		acc += arg
	pc += 1

print('Stopped infinite loop just in time! Accumulator value:', acc)