s = ""
with open("./input.txt", "r") as f:
    s = f.read()


find = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "zero",
    *list("1234567890"),
]

num_to_digit = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
    "zero": "0",
}


def first(s: str, find: list[str]):
    min_i = len(s)
    ret = ""
    for f in find:
        i = s.find(f)
        if i != -1 and i < min_i:
            min_i = i
            ret = f
    return ret


def last(s: str, find: list[str]):
    s = s[::-1]
    find2 = [f[::-1] for f in find]
    return first(s, find2)[::-1]


def num(l):
    digit1 = first(l, find)
    digit2 = last(l, find)
    if digit1 in num_to_digit:
        digit1 = num_to_digit[digit1]
    if digit2 in num_to_digit:
        digit2 = num_to_digit[digit2]
    return int(digit1 + digit2)


digits = [num(l) for l in s.split("\n")]

print(sum(digits))
