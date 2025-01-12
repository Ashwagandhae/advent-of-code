import math, random, time
import numpy as np

s = ""
with open("./input.txt", "r") as f:
    s = f.read()

arr1 = []
arr2 = []
for line in s.split("\n"):
    [x, y] = line.split("   ")
    arr1.append(int(x))
    arr2.append(int(y))

arr1.sort()
arr2.sort()


answer = 0

for i in range(len(arr1)):
    answer += abs(arr1[i] - arr2[i])
print(answer)
