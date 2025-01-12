import math, random, time
import numpy as np

s = ""
with open("./input.txt", "r") as f:
    s = f.read()
answer = 0
grid = list(map(list, s.split("\n")))


def add_pos(pos1, pos2):
    return (pos1[0] + pos2[0], pos1[1] + pos2[1])


def get_pos(grid, pos) -> tuple[int, int] | None:
    if pos[0] >= 0 and pos[0] < len(grid) and pos[1] >= 0 and pos[1] < len(grid[0]):
        return grid[pos[0]][pos[1]]
    return None


def check_xmas(grid, pos, delta):
    for char in list("XMAS"):
        if get_pos(grid, pos) != char:
            return False
        pos = add_pos(pos, delta)
    return True


for i in range(len(grid)):
    for j in range(len(grid[0])):
        seen = False
        for delta in [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]:
            if check_xmas(grid, (i, j), delta):
                answer += 1
                seen = True
        if seen:
            print(grid[i][j], end="")
        else:
            print(".", end="")
    print()

print(answer)
