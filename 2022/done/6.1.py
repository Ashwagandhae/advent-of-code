with open("../data/6.txt", "r") as f:
    data = f.read()

chars = []
i = 0
for char in data:
    i += 1
    chars.append(char)
    if len(chars) > 4:
        chars.pop(0)
        # if all chars are diff
        if len(set(chars)) == len(chars):
            print(i)
            break
