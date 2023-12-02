import json

with open("../data/13.txt", "r") as f:
    raw = f.read()

data = []
for block in raw.split("\n\n"):
    current = []
    for line in block.split("\n"):
        current.append(json.loads(line))
    data.append(current)


def compare(left, right):
    # if both are ints, compare them
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            return 1
        elif left > right:
            return -1
        else:
            return 0
    # if both are lists, compare them
    elif isinstance(left, list) and isinstance(right, list):
        for i in range(len(left)):
            if i >= len(right):
                return -1
            result = compare(left[i], right[i])
            if result != 0:
                return result
        if len(left) < len(right):
            return 1
        elif len(right) > len(left):
            return -1
        else:
            return 0
    # if right is int and left is list
    elif isinstance(left, list) and isinstance(right, int):
        return compare(left, [right])
    # if left is int and right is list
    elif isinstance(left, int) and isinstance(right, list):
        return compare([left], right)


correct_pairs = []
correct_sum = 0
for (i, pair) in enumerate(data):
    if compare(pair[0], pair[1]) == 1:
        correct_pairs.append(pair)
        print("correct")
        correct_sum += i + 1
    else:
        print("incorrect")


print(correct_sum)
