s = ""
with open("./input.txt", "r") as f:
    s = f.read()


arr = s.split("\n")


def get(x, y):
    if x < 0 or y < 0:
        return "."
    if y >= len(arr):
        return "."
    if x >= len(arr[0]):
        return "."
    return arr[y][x]


def look_around(x, y):
    return [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]


num = "1234567890"
sum = 0
gears = {}
for j in range(len(arr)):
    for i in range(len(arr[0])):
        if get(i, j) in num and get(i - 1, j) not in num:
            num_parts = [(i, j)]
            new_x = i + 1
            while get(new_x, j) in num:
                num_parts.append((new_x, j))
                new_x += 1
            look_around_parts = []
            for x, y in num_parts:
                look_around_parts += look_around(x, y)
            look_around_parts = list(set(look_around_parts))
            numm = int("".join([get(x, y) for x, y in num_parts]))
            for x, y in look_around_parts:
                if get(x, y) == "*":
                    if (x, y) not in gears:
                        gears[(x, y)] = []
                    gears[(x, y)].append(numm)

print(gears)
for x, y in gears.keys():
    if len(gears[(x, y)]) == 2:
        sum += gears[(x, y)][0] * gears[(x, y)][1]
print(sum)
