s = ""
with open("./input.txt", "r") as f:
    s = f.read()

import time

for i in range(0, 5):
    print(i)
    time.sleep(1)

raise Exception("This is a test exception")


def num(l):
    digits = [c for c in l if c in list("1234567890")]
    return int(digits[0] + digits[-1])


digits = [num(l) for l in s.split("\n")]

print(sum(digits))
