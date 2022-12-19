#!/usr/bin/python3
import sys
from operator import sub, add
from math import sqrt

lines = sys.stdin.readlines()

width = len(lines[0].strip())
height = len(lines)

coords = [[0 for _ in range(width)]  for _ in range(height)]

start_coord = (0,0)
end_coord = (0,0)

look_directions = [(1,0), (-1,0), (0,1), (0,-1)]

def get_score(end_coord: int, start_coord: int, dx: int, dy: int):
    dir_x, dir_y = tuple(map(sub,end_coord,tuple(map(add, (dx, dy), start_coord))))
    return sqrt(dir_x**2 + dir_y**2)

def test_directions(elevation: int, x: int, y: int) -> tuple:
    ones = []
    zeroes = []
    for dx, dy in look_directions:
        if y+dy>=0 and x+dx>=0 and y+dy<height and x+dx<width:
            dir = coords[y+dy][x+dx]-elevation
            if dir == 1:
                ones.append(((dx, dy), get_score(end_coord, start_coord, dx, dy)))
            elif dir == 0:
                zeroes.append(((dx, dy), get_score(end_coord, start_coord, dx, dy)))
    dir = (0,0)
    max_val = 1000
    if len(ones)>0:
        for cord, score in ones:
            if score<max_val:
                max_val = score
                dir=cord
    else:
        for cord, score in zeroes:
            if score<max_val:
                max_val = score
                dir=cord

    return dir

for y, line in enumerate(lines):
    for x, ch in enumerate(line.strip()):
        coords[y][x] = ord(ch)
        if ch == 'S':
            start_coord = (x,y)
            coords[y][x] = ord('a')
            print(f"x: {x} and y: {y} for 'S'. {start_coord}")
        elif ch == 'E':
            end_coord = (x,y)
            coords[y][x] = ord('z')
            print(f"x: {x} and y: {y} for 'E'. {end_coord}")

for _ in range(40):
    (x,y) = start_coord
    start_coord = tuple(map(add,test_directions(coords[y][x], x, y), start_coord))
    print(start_coord)

#for y, row in enumerate(coords):
    #for x, elevation in enumerate(row):

        #print(f"{(x,y)} has elevation {elevation}")

        #if start_coord == (x,y):
        #    print(elevation)

    