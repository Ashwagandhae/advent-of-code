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

arrMap = {}
for x in arr2:
    if x in arrMap:
        arrMap[x] += 1
    else:
        arrMap[x] = 1


answer = 0

for x in arr1:
    if x in arrMap:
        answer += x * arrMap[x]

# for i in range(len(arr1)):
#     answer += abs(arr1[i] - arr2[i])
print(answer)
