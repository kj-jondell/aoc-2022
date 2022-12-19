import sys

lines = sys.stdin.readlines()

for line in lines:
    coords = [tuple(pair.split(",")) for pair in line.strip().split(" -> ")]
    base_line = coords[0][0]
    for cord in coords:
        if cord[0]<base_line:
            base_line = cord[0]
        print(f"{cord} {base_line}", end="")
print()
