s = ""
with open("./input.txt", "r") as f:
    s = f.read()
answer = 0


def nums_safe(nums):
    decreasing = True
    increasing = True
    maxdiff = 0
    for i in range(0, len(nums) - 1):
        if nums[i] >= nums[i + 1]:
            decreasing = False
        if nums[i] <= nums[i + 1]:
            increasing = False
        maxdiff = max(maxdiff, abs(nums[i] - nums[i + 1]))
    return (increasing or decreasing) and maxdiff <= 3


for line in s.split("\n"):
    nums = list(map(int, line.split(" ")))
    success = False
    for remove_i in range(0, len(nums)):
        if nums_safe(nums[:remove_i] + nums[remove_i + 1 :]):
            success = True
    if success:
        answer += 1
print(answer)
