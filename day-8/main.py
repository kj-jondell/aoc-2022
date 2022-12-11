import sys

lines = sys.stdin.readlines()

coords = []
for line in lines:
    new_coords = []
    for new_height in line.strip(): # x-coord
        new_coords.append(int(new_height))
    coords.append(new_coords)

width = len(new_coords) 
height = len(coords)

invisible_trees = 0

for x in range(1, width-1): #dont bother with edges
    for y in range(1, height-1):
        invisible_count = 0
        for index, range_n in enumerate([range(0, x), range(x+1, width), range(0, y), range(y+1, height)]): #look left, look right, look up, look down
            for cmp in range_n: 
                cmp_x = x if index >= 2 else cmp
                cmp_y = y if index < 2 else cmp
                if coords[y][x]<=coords[cmp_y][cmp_x]: 
                    invisible_count += 1
                    break
        if invisible_count == 4:
            invisible_trees += 1

print(f"{width*height-invisible_trees} trees are visible")