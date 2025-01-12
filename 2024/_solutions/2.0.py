import math, random, time
import numpy as np

s = ""
with open("./input.txt", "r") as f:
    s = f.read()
answer = 0

for line in s.split("\n"):
    decreasing = True
    increasing = True
    maxdiff = 0
    nums = list(map(int, line.split(" ")))
    for i in range(0, len(nums) - 1):
        if nums[i] >= nums[i + 1]:
            decreasing = False
        if nums[i] <= nums[i + 1]:
            increasing = False
        maxdiff = max(maxdiff, abs(nums[i] - nums[i + 1]))
    if (increasing or decreasing) and maxdiff <= 3:
        answer += 1
print(answer)
