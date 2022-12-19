#!/usr/bin/python3
import sys

class Monkey:

    def __init__(self, starting_items: list[int], operation: str, test_devisor: int, true_monkey: int, false_monkey: int) -> None:
        self.starting_items = starting_items
        self.operation = operation
        self.test_devisor = test_devisor
        self.true_monkey = true_monkey
        self.false_monkey = false_monkey
        self.inspect_times = 0

    def incr_inspect(self):
        self.inspect_times += 1

    def get_inspect_times(self):
        return self.inspect_times

    def __str__(self):
        return f"Monkey has starting items {self.starting_items} and operation {self.operation}. If divisible by {self.test_devisor} then throw to {self.true_monkey} else {self.false_monkey}"

    def replace_item(self, new_item):
        self.starting_items[0] = new_item

    def get_item(self) -> int:
        if len(self.starting_items) > 0:
            return self.starting_items.pop(0)
        else:
            return -1
    
    def add_item(self, new_item: int):
        self.starting_items.append(new_item)

lines = sys.stdin.readlines()

worry_level = 0
monkies = []

for index, line in enumerate(lines):
    if "Operation: " in line:
        command = line.strip().split("Operation: ")[1]
        #old = new
        #exec(command)
    elif "Starting items: " in line:
        starting_items = [int(ch) for ch in line.strip().split("Starting items: ")[1].split(", ")]
    elif "Test: " in line:
        test_divisor = int(line.strip().split("Test: divisible by ")[1])
    elif "If true: throw to monkey " in line:
        true_monkey = int(line.strip().split("If true: throw to monkey ")[1])
    elif "If false: throw to monkey " in line:
        false_monkey = int(line.strip().split("If false: throw to monkey ")[1])
    elif len(line.strip()) == 0:
        monkies.append(Monkey(starting_items, command, test_divisor, true_monkey, false_monkey))

monkies.append(Monkey(starting_items, command, test_divisor, true_monkey, false_monkey))
#for index, monkey in enumerate(monkies):  
#    print(f"Monkey {index}: {monkey}")

# For all monkeys that exists
#     For all items in list:
#         Monkey inspects an item with a worry level of 79.
#             Worry level is multiplied by 19 to 1501.
#             Monkey gets bored with item. Worry level is divided by 3 to 500.
#             Current worry level is not divisible by 23.
#             Item with worry level 500 is thrown to monkey 3.

new = 0
for round in range(10000):
    if round%20 == 0:
        print(f"Doing round {round}.") 
        #print(f"Len {len(monkies[0].starting_items)}")
    for monkey in monkies:
        for item in monkey.starting_items.copy():
            monkey.incr_inspect()
            # inspect
            #print(f"Monkey inspects an item with a worry level of {item}")
            old = item
            # command
            #print(f"Worry level before operation is {old}")
            exec(monkey.operation)

            #print(f"Worry level after operation {monkey.operation} is {new}")
            # divide by three
            #new = int(new/3)
            #print(f"Monkey gets bored with item. Worry level is divided by 3 to {new}.")
            monkey.replace_item(new)
            # test

            if int(new) % monkey.test_devisor == 0:
                monkies[0].starting_items.append(monkey.starting_items.pop(0))
            else:
                pass
                #monkies[0].starting_items.append(monkey.starting_items.pop(0))
            #    #print(f"Item {new} is divisible by {monkey.test_devisor} throw to monkey {monkey.true_monkey}.")
            #    monkies[monkey.true_monkey].add_item(monkey.get_item())
            #else:
            #    #print(f"Item {new} is not divisible by {monkey.test_devisor} throw to monkey {monkey.false_monkey}.")
            #    monkies[monkey.false_monkey].add_item(monkey.get_item())

            # throw
        #print("-----")

inspect_times = []
for index, monkey in enumerate(monkies):
    print(f"Monkey {index} inspected items {monkey.get_inspect_times()} times")
    inspect_times.append(monkey.get_inspect_times())

inspect_times.sort()
print(inspect_times[-2]*inspect_times[-1])