#!/usr/bin/python3
import sys

lines = sys.stdin.readlines()

new = 0
old = 1
for line in lines:
    if "Operation: " in line:
        command = line.strip().split("Operation: ")[1]
        old = new
        exec(command)
        #print(new)
    elif "Starting items: " in line:
        starting_items = [int(ch) for ch in line.strip().split("Starting items: ")[1].split(", ")]
        print(starting_items)