with open("../data/10.txt", "r") as f:
    data = f.read()

strengths = []
cycle = 1
signal = 1

for line in data.splitlines():
    line = line.split()
    if line[0] == "addx":
        cycle += 1
        if cycle - 20 >= 0 and (cycle - 20) % 40 == 0:
            strengths.append(signal * cycle)
        cycle += 1
        signal += int(line[1])
    elif line[0] == "noop":
        cycle += 1
    if cycle - 20 >= 0 and (cycle - 20) % 40 == 0:
        strengths.append(signal * cycle)
print(strengths)
print(sum(strengths))
