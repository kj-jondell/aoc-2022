#!/usr/bin/python3
import sys, ast

def comp_lists(first_comp: list, second_comp: list) -> bool:
    if len(second_comp) == 0:
        return False

    comp_len = min(len(first_comp), len(second_comp))

    for i in range(comp_len):
        #print(type(first_comp[i]),type(second_comp[i]))
        if type(first_comp[i]) == int and type(second_comp[i]) == int:
            if first_comp[i]>second_comp[i]:
                return False
        elif type(first_comp[i]) == list and type(second_comp[i]) == list:
            if not comp_lists(first_comp[i], second_comp[i]):
                return False
        elif type(first_comp[i]) != type(second_comp[i]):
            if type(first_comp[i]) == int:
                if not comp_lists([first_comp[i]], second_comp[i]):
                    return False
            else:
                if not comp_lists(first_comp[i], [second_comp[i]]):
                    return False
    return True

lines = sys.stdin.readlines()

#lines = lines[0:5]

indeces = []

for index in range(0, len(lines), 3):
    first_comp = ast.literal_eval(lines[index])
    second_comp = ast.literal_eval(lines[index+1])

    if len(first_comp)<=len(second_comp):
        if comp_lists(first_comp,second_comp):
            print(int(index/3+1))
            indeces.append(int(index/3+1))
    #else:
    #    print(int(index/3+1),False, first_comp, second_comp,"false", len(first_comp), len(second_comp))

print(f"sum: {sum(indeces)}")