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


a = 0
things = s.split("mul(")
for thing in things:
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
