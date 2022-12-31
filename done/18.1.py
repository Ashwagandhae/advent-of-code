import numpy as np

with open("../data/18.txt", "r") as f:
    raw = f.read()

cubes = []
for line in raw.splitlines():
    pos = [int(s) for s in line.split(",")]
    cubes.append(np.array(pos))


sides_adder = [
    # bottom
    [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]],
    # top
    [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]],
    # left
    [[0, 0, 0], [0, 1, 0], [0, 1, 1], [0, 0, 1]],
    # right
    [[1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 0, 1]],
    # back
    [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]],
    # front
    [[0, 1, 0], [1, 1, 0], [1, 1, 1], [0, 1, 1]],
]


def get_sides(pos):
    sides = []
    # get 6 sides (array of 4 positions) of 1x1x1 cube at pos
    for side_adder in sides_adder:
        side = []
        for adder in side_adder:
            side.append(pos + np.array(adder))
        sides.append(side)
    return sides


def side_to_string(side):
    pos_string = [",".join([str(p) for p in pos]) for pos in side]
    ret = "-".join(sorted(pos_string))
    return ret


side_dict = {}
for cube in cubes:
    sides = get_sides(cube)
    for side in sides:
        side_key = side_to_string(side)
        if side_key in side_dict:
            side_dict[side_key] += 1
        else:
            side_dict[side_key] = 1
surface_area = 0
for key, value in side_dict.items():
    if value == 1:
        surface_area += 1
print(surface_area)
