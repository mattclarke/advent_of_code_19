from copy import deepcopy

with open("input_data.txt") as f:
    input_data = f.read()

ORIG_DATA = []
for row in input_data.split("\n"):
    r = [c for c in row if c != "\n"]
    ORIG_DATA.append(r)


def print_data(data):
    for y in range(len(data)):
        print("".join(data[y]))
    # print(calc_hash(data))
    print()


def calc_hash(data):
    result = 0
    addition = 1
    for r in data:
        for c in r:
            if c == "#":
                result += addition
            addition = addition * 2

    return result


print_data(ORIG_DATA)
x_dirs = [-1, 1, 0, 0]
y_dirs = [0, 0, -1, 1]

PREV_DATA = deepcopy(ORIG_DATA)
NEW_DATA = deepcopy(PREV_DATA)
HASHES = set()

# Part 1 = 18370591
while True:
    for y in range(len(PREV_DATA)):
        for x in range(len(PREV_DATA[0])):
            adj = 0
            for nx, ny in zip(x_dirs, y_dirs):
                if x + nx < 0 or x + nx >= len(PREV_DATA[0]):
                    continue
                if y + ny < 0 or y + ny >= len(PREV_DATA):
                    continue
                if PREV_DATA[y + ny][x + nx] == "#":
                    adj += 1
            if PREV_DATA[y][x] == "#":
                if adj == 1:
                    NEW_DATA[y][x] = "#"
                else:
                    NEW_DATA[y][x] = "."
            else:
                if adj == 1 or adj == 2:
                    NEW_DATA[y][x] = "#"
                else:
                    NEW_DATA[y][x] = "."

    # print_data(NEW_DATA)
    PREV_DATA = deepcopy(NEW_DATA)
    hash = calc_hash(NEW_DATA)
    if hash in HASHES:
        print(f"Result = {hash}")
        break
    else:
        HASHES.add(hash)


# PART 2 = 2040
ORIG_DATA = deepcopy(ORIG_DATA)
ORIG_DATA[2][2] = "?"

LEVELS = {
    0: deepcopy(ORIG_DATA)
}

PREV_LEVELS = deepcopy(LEVELS)
HIGHEST_LEVEL = 0


def recurse_down(level, n, e, s, w):
    global HIGHEST_LEVEL
    HIGHEST_LEVEL = max(HIGHEST_LEVEL, level)

    if level in LEVELS:
        PREV_DATA = PREV_LEVELS[level]
    else:
        PREV_DATA = [[".", ".", ".", ".", "."] for i in range(5)]
        PREV_DATA[2][2] = "?"
    NEW_DATA = deepcopy(PREV_DATA)

    for y in range(len(PREV_DATA)):
        for x in range(len(PREV_DATA[0])):
            if x == 2 and y == 2:
                continue
            adj = 0
            # Check outside
            if x == 0:
                if w:
                    adj += 1
            elif x == len(PREV_DATA[0]) - 1:
                if e:
                    adj += 1
            if y == 0:
                if n:
                    adj += 1
            elif y == len(PREV_DATA) - 1:
                if s:
                    adj += 1

            # Check inside
            if x == 2 and y == 1:
                if level - 1 in PREV_LEVELS:
                    adj += PREV_LEVELS[level - 1][0].count("#")
            elif x == 2 and y == 3:
                if level - 1 in PREV_LEVELS:
                    adj += PREV_LEVELS[level - 1][-1].count("#")
            elif y == 2 and x == 1:
                if level - 1 in PREV_LEVELS:
                    sub = PREV_LEVELS[level - 1]
                    for i in range(len(sub)):
                        if sub[i][0] == "#":
                            adj += 1
            elif y == 2 and x == 3:
                if level - 1 in PREV_LEVELS:
                    sub = PREV_LEVELS[level - 1]
                    for i in range(len(sub)):
                        if sub[i][-1] == "#":
                            adj += 1

            for nx, ny in zip(x_dirs, y_dirs):
                # Ignore 2, 2
                if x + nx == 2 and y + ny == 2:
                    continue

                # Out of bounds
                if x + nx < 0 or x + nx >= len(PREV_DATA[0]):
                    continue
                if y + ny < 0 or y + ny >= len(PREV_DATA[0]):
                    continue

                if PREV_DATA[y + ny][x + nx] == "#":
                    adj += 1

            if y == 2 and x == 2:
                pass
            elif PREV_DATA[y][x] == "#":
                if adj == 1:
                    NEW_DATA[y][x] = "#"
                else:
                    NEW_DATA[y][x] = "."
            else:
                if adj == 1 or adj == 2:
                    NEW_DATA[y][x] = "#"
                else:
                    NEW_DATA[y][x] = "."
    LEVELS[level] = deepcopy(NEW_DATA)
    n = True if PREV_DATA[1][2] == "#" else False
    s = True if PREV_DATA[3][2] == "#" else False
    w = True if PREV_DATA[2][1] == "#" else False
    e = True if PREV_DATA[2][3] == "#" else False
    if any([n,s,w,e]) or level - 1 in LEVELS:
        recurse_down(level - 1, n, e, s, w)


def recurse_up(level, n, e, s, w):
    global HIGHEST_LEVEL
    HIGHEST_LEVEL = max(HIGHEST_LEVEL, level)

    if level in LEVELS:
        PREV_DATA = PREV_LEVELS[level]
    else:
        PREV_DATA = [[".", ".", ".", ".", "."] for i in range(5)]
        PREV_DATA[2][2] = "?"
    NEW_DATA = deepcopy(PREV_DATA)

    for y in range(len(PREV_DATA)):
        for x in range(len(PREV_DATA[0])):
            if x == 2 and y == 2:
                continue

            adj = 0

            # Special cases around centre
            if x == 2 and y == 1:
                adj += n
            elif x == 2 and y == 3:
                adj += s
            elif x == 1 and y == 2:
                adj += w
            elif x == 3 and y == 2:
                adj += e

            for nx, ny in zip(x_dirs, y_dirs):
                # Ignore 2, 2
                if x + nx == 2 and y + ny == 2:
                    continue

                # Out of bounds
                if x + nx < 0 or x + nx >= len(PREV_DATA[0]):
                    continue
                if y + ny < 0 or y + ny >= len(PREV_DATA):
                    continue

                if PREV_DATA[y + ny][x + nx] == "#":
                    adj += 1

            if y == 2 and x == 2:
                pass
            elif PREV_DATA[y][x] == "#":
                if adj == 1:
                    NEW_DATA[y][x] = "#"
                else:
                    NEW_DATA[y][x] = "."
            else:
                if adj == 1 or adj == 2:
                    NEW_DATA[y][x] = "#"
                else:
                    NEW_DATA[y][x] = "."
    LEVELS[level] = deepcopy(NEW_DATA)


def count_bugs(data):
    result = 0
    for r in data:
        result += r.count("#")
    return result


for i in range(200):
    recurse_down(HIGHEST_LEVEL, False, False, False, False)

    # Count how many #s in highest's edges
    n = PREV_LEVELS[HIGHEST_LEVEL][0].count("#")
    s = PREV_LEVELS[HIGHEST_LEVEL][-1].count("#")
    e = 0
    w = 0
    for r in PREV_LEVELS[HIGHEST_LEVEL]:
        if r[0] == "#":
            w += 1
        if r[-1] == "#":
            e += 1

    if n + e + s + w > 0:
        recurse_up(HIGHEST_LEVEL + 1, n, e, s, w)

    PREV_LEVELS = deepcopy(LEVELS)


# keys = list(LEVELS.keys())
# for k in sorted(keys, reverse=True):
#     print(f"Level: {k}")
#     print_data(LEVELS[k])

result = 0
for lvl in LEVELS.values():
    result += count_bugs(lvl)

print(f"Result = {result}")

