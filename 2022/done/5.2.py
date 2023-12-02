with open("../data/5.txt", "r") as f:
    data = f.read()
crate_stacks = [
    [],
    [],
    [],
    [],
    [],
    [],
    [],
    [],
    [],
]
for line in data.split("\n")[:8]:
    for i in range(0, len(line), 4):
        if line[i + 1] != " ":
            crate_stacks[i // 4].append(line[i + 1])

print(crate_stacks)
for inst in data.split("\n")[10:]:
    inst = inst.replace("move ", "")
    amount = int(inst.split(" from ")[0])
    _from = int(inst.split(" from ")[1].split(" to ")[0]) - 1
    _to = int(inst.split(" to ")[1]) - 1
    move = crate_stacks[_from][:amount]
    crate_stacks[_from] = crate_stacks[_from][amount:]
    # move.reverse()
    crate_stacks[_to] = move + crate_stacks[_to]
for i, stack in enumerate(crate_stacks):
    print(stack[0], end="")
