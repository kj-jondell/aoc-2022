import sys

input = sys.stdin.readlines()

splitted = [obj for line in input for obj in line.strip().split()]

register_x = 1
signal_strength = 0

for cycle, row in enumerate(splitted):
    if (cycle+1) % 40 == 20:
        signal_strength += register_x*(cycle+1)
    if row != 'noop' and row != 'addx':
        register_x += int(row)

print(f"Signal strength after {cycle} cycles is {signal_strength}")

