with open("../data/10.txt", "r") as f:
    data = f.read()

cycle = -1
sprite_pos = 1

screen = []


def drawSprite(sprite_pos, cycle):

    global screen
    screen_pos = cycle % 40

    print()
    if (
        sprite_pos - 1 == screen_pos
        or sprite_pos == screen_pos
        or sprite_pos + 1 == screen_pos
    ):
        screen.append("#")
    else:
        screen.append(".")
    print("cycle", cycle, screen_pos)
    print("sprite_pos", sprite_pos)
    print("".join(screen))


def updateCycle():
    global cycle, sprite_pos
    cycle += 1
    drawSprite(sprite_pos, cycle)


for line in data.splitlines():
    line = line.split()
    if line[0] == "addx":
        updateCycle()
        updateCycle()
        sprite_pos += int(line[1])
    elif line[0] == "noop":
        updateCycle()
for i, pixel in enumerate(screen):
    if i % 40 == 0:
        print()
    print(pixel, end="")
