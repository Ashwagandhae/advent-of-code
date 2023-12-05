s = ""
with open("./input.txt", "r") as f:
    s = f.read()


arr = s.split("\n")


def get(x, y):
    if x < 0 or y < 0:
        return "."
    if y >= len(arr):
        return "."
    if x >= len(arr[y]):
        return "."
    return arr[y][x]


def look_around(x, y):
    return [
        get(x - 1, y - 1),
        get(x, y - 1),
        get(x + 1, y - 1),
        get(x - 1, y),
        get(x + 1, y),
        get(x - 1, y + 1),
        get(x, y + 1),
        get(x + 1, y + 1),
    ]


num = "1234567890"
sum = 0
for y in range(len(arr)):
    for x in range(len(arr[0])):
        if get(x, y) in num and get(x - 1, y) not in num:
            num_parts = [(x, y)]
            new_x = x + 1
            while get(new_x, y) in num:
                num_parts.append((new_x, y))
                new_x += 1
            look_around_parts = []
            for x, y in num_parts:
                look_around_parts += look_around(x, y)
            if any([l != "." and l not in num for l in look_around_parts]):
                sum += int("".join([get(x, y) for x, y in num_parts]))

print(sum)
