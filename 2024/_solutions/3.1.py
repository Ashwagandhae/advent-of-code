s = None
with open("./input.txt", "r") as f:
    s = f.read()


def extract_num(chars):
    numstr = ""
    while len(chars) > 0 and chars[0] in "1234567890":
        numstr += chars[0]
        chars = chars[1:]
    if len(numstr) == 0:
        return [], False, 0
    return chars, True, int(numstr)


bitmap = []
for i in range(len(s)):
    if s[i : i + 4] == "do()":
        bitmap.append(True)
    elif s[i : i + 7] == "don't()":
        bitmap.append(False)
    elif len(bitmap) == 0:
        bitmap.append(True)
    else:
        bitmap.append(bitmap[-1])

a = 0
things = s.split("mul(")
i = 0
for thing in things:
    oldi = i
    i += len(thing) + 4
    if not bitmap[oldi]:
        continue
    chars = list(thing)
    chars, success, num = extract_num(chars)
    if not success:
        continue
    if not chars[0] == ",":
        continue
    chars = chars[1:]
    chars, success, num1 = extract_num(chars)
    if not success:
        continue
    if not chars[0] == ")":
        continue
    print(num, num1)
    a += num * num1


print(a)
