s = ""
with open("./input.txt", "r") as f:
    s = f.read()


def num(l):
    digits = [c for c in l if c in list("1234567890")]
    return int(digits[0] + digits[-1])


digits = [num(l) for l in s.split("\n")]

print(sum(digits))
