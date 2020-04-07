with open("input_data.txt") as f:
    input_data = f.read()

MAX_X = 0
ORIG_DATA = []
for row in input_data.split("\n"):
    r = [c for c in row if c != "\n"]
    MAX_X = max(MAX_X, len(r))
    ORIG_DATA.append(r)

PORTALS = {}
REVERSE_LOOKUP = {}

for y in range(len(ORIG_DATA)):
    while len(ORIG_DATA[y]) < MAX_X:
        ORIG_DATA[y].append(" ")
    for x in range(MAX_X):
        if ORIG_DATA[y][x].isupper():
            if ORIG_DATA[y][x + 1].isupper():
                if x == 0 or ORIG_DATA[y][x - 1] == " ":
                    if x < MAX_X / 2 :
                        ORIG_DATA[y][x + 1] = ORIG_DATA[y][x] + ORIG_DATA[y][x + 1]
                    else:
                        ORIG_DATA[y][x + 1] = ORIG_DATA[y][x] + ORIG_DATA[y][x + 1] + "!"
                    ORIG_DATA[y][x] = " "
                    PORTALS[ORIG_DATA[y][x + 1]] = (x + 2, y)
                    REVERSE_LOOKUP[(x + 2, y)] = ORIG_DATA[y][x + 1]
                else:
                    if x < MAX_X / 2:
                        ORIG_DATA[y][x] = ORIG_DATA[y][x] + ORIG_DATA[y][x + 1] + "!"
                    else:
                        ORIG_DATA[y][x] = ORIG_DATA[y][x] + ORIG_DATA[y][x + 1]
                    ORIG_DATA[y][x + 1] = " "
                    PORTALS[ORIG_DATA[y][x]] = (x - 1, y)
                    REVERSE_LOOKUP[(x - 1, y)] = ORIG_DATA[y][x]
            elif y > 0 and ORIG_DATA[y - 1][x].isupper():
                if y - 2 < 0 or ORIG_DATA[y - 2][x] == " ":
                    if y < len(ORIG_DATA) / 2:
                        ORIG_DATA[y][x] = ORIG_DATA[y-1][x] + ORIG_DATA[y][x]
                    else:
                        ORIG_DATA[y][x] = ORIG_DATA[y-1][x] + ORIG_DATA[y][x] + "!"
                    ORIG_DATA[y-1][x] = " "
                    if ORIG_DATA[y][x] in PORTALS:
                        ORIG_DATA[y][x] += "!"
                    PORTALS[ORIG_DATA[y][x]] = (x, y + 1)
                    REVERSE_LOOKUP[(x, y+1)] = ORIG_DATA[y][x]
                else:
                    if y < len(ORIG_DATA) / 2:
                        ORIG_DATA[y-1][x] = ORIG_DATA[y-1][x] + ORIG_DATA[y][x] + "!"
                    else:
                        ORIG_DATA[y-1][x] = ORIG_DATA[y-1][x] + ORIG_DATA[y][x]
                    ORIG_DATA[y][x] = " "
                    if ORIG_DATA[y-1][x] in PORTALS:
                        ORIG_DATA[y-1][x] += "!"
                    PORTALS[ORIG_DATA[y-1][x]] = (x, y - 2)
                    REVERSE_LOOKUP[(x, y-2)] = ORIG_DATA[y-1][x]


for y in range(len(ORIG_DATA)):
    print(ORIG_DATA[y])

# Part 1 = 590
START = PORTALS["AA"]
END = PORTALS["ZZ"]
del PORTALS["AA"]
del PORTALS["ZZ"]
del REVERSE_LOOKUP[START]
del REVERSE_LOOKUP[END]

GRAPH = {}
DIRS = [(-1, 0), (1, 0), (0, -1), (0, 1)]
QUEUE = [(START, 0)]

while QUEUE:
    current, dist = QUEUE.pop(0)
    if current in GRAPH and dist > GRAPH[current]:
        continue
    GRAPH[current] = dist
    for x_dir, y_dir in zip([-1, 1, 0, 0], [0, 0, -1, 1]):
        new_x = current[0] + x_dir
        new_y = current[1] + y_dir
        if ORIG_DATA[new_y][new_x] == ".":
            if (new_x, new_y) in REVERSE_LOOKUP:
                portal = REVERSE_LOOKUP[(new_x, new_y)]
                op_portal = portal[:2] if portal.endswith("!") else portal + "!"
                p_x, p_y = PORTALS[op_portal]
                QUEUE.append(((p_x, p_y), dist + 2))
            else:
                QUEUE.append(((new_x, new_y), dist + 1))

print(f"Part 1  =  {GRAPH[END]}")

# Part 2 = 7180
from heapq import *

GRAPH = {}
DIRS = [(-1, 0), (1, 0), (0, -1), (0, 1)]
QUEUE = [(0, (START, 0))]

while QUEUE:
    level, (current, dist) = heappop(QUEUE)
    if level == 0 and current == END:
        print(f"Part 2 = {dist}")
        break
    if (current, level) in GRAPH and dist > GRAPH[(current, level)]:
        continue
    GRAPH[(current, level)] = dist
    for x_dir, y_dir in zip([-1, 1, 0, 0], [0, 0, -1, 1]):
        new_x = current[0] + x_dir
        new_y = current[1] + y_dir
        if ORIG_DATA[new_y][new_x] == ".":
            if (new_x, new_y) in REVERSE_LOOKUP:
                # If portal ends in ! then we are recursing
                portal = REVERSE_LOOKUP[(new_x, new_y)]
                if portal.endswith("!"):
                    op_portal = portal[:2]
                    p_x, p_y = PORTALS[op_portal]
                    heappush(QUEUE, (level + 1, ((p_x, p_y), dist + 2)))
                elif level > 0:
                    op_portal = portal + "!"
                    p_x, p_y = PORTALS[op_portal]
                    heappush(QUEUE, (level - 1, ((p_x, p_y),  dist + 2)))
            else:
                heappush(QUEUE, (level, ((new_x, new_y), dist + 1)))
