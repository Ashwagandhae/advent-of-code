import numpy as np


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


def surface_area_from_cubes(cubes):
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
    return surface_area


def out_of_bounds(cube_arr, pos):
    if pos[0] < 0 or pos[1] < 0 or pos[2] < 0:
        return True
    if (
        pos[0] >= cube_arr.shape[0]
        or pos[1] >= cube_arr.shape[1]
        or pos[2] >= cube_arr.shape[2]
    ):
        return True
    return False


def get_air_pocket_cubes(cube_arr, pos, escapable_poses):
    queue = [pos]
    visited = set()
    count = 0
    while queue:
        pos = queue.pop(0)
        if tuple(pos) in visited:
            continue
        count += 1
        visited.add(tuple(pos))
        for pos_adder in [
            [0, 0, 1],
            [0, 0, -1],
            [0, 1, 0],
            [0, -1, 0],
            [1, 0, 0],
            [-1, 0, 0],
        ]:
            new_pos = pos + np.array(pos_adder)
            if out_of_bounds(cube_arr, new_pos) or tuple(new_pos) in escapable_poses:
                # escape found
                return None
            if cube_arr[new_pos[0], new_pos[1], new_pos[2]] == False:
                queue.append(new_pos)
    return visited


with open("../data/18.txt", "r") as f:
    raw = f.read()

cubes = []
for line in raw.splitlines():
    pos = [int(s) for s in line.split(",")]
    cubes.append(np.array(pos))

surface_area = surface_area_from_cubes(cubes)

cube_arr = np.zeros((25, 25, 25), dtype=bool)
for cube in cubes:
    cube_arr[cube[0], cube[1], cube[2]] = True

escapable_poses = set()
pocket_poses = set()
# loop through all np array
progress = 0
for x in range(cube_arr.shape[0]):
    for y in range(cube_arr.shape[1]):
        for z in range(cube_arr.shape[2]):
            progress += 1
            if progress % 100 == 0:
                print(f"progress: {progress}/{cube_arr.size}")
            # only check air cubes
            if cube_arr[x, y, z]:
                continue
            # don't check already checked cubes
            if (x, y, z) in pocket_poses:
                continue
            pos = np.array([x, y, z])
            air_pocket_cubes = get_air_pocket_cubes(cube_arr, pos, escapable_poses)
            # if can escape, add to escapable_poses
            if air_pocket_cubes == None:
                escapable_poses.add(tuple(pos))
            else:
                surface_area -= surface_area_from_cubes(air_pocket_cubes)
                # add to pocket_poses
                pocket_poses.update(air_pocket_cubes)


print(surface_area)
