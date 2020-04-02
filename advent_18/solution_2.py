from collections import deque

# Part 2
class Node:
    def __init__(self, content=None):
        self.links = []
        self.content = content


with open("input_data_2.txt") as f:
    input_data = f.read()

ORIG_DATA = []
for row in input_data.split():
    print(row)
    r = [c for c in row if c != "\n"]
    ORIG_DATA.append(r)

KEYS = {}
GATES = {}
LOOKUP = {}
START = []
NODES = {}

for y in range(len(ORIG_DATA)):
    for x in range(len(ORIG_DATA[0])):
        if ORIG_DATA[y][x] != "#":
            if ORIG_DATA[y][x] == "@":
                START.append((x, y))
                n = Node("@")
            elif ORIG_DATA[y][x].isalpha():
                n = Node(ORIG_DATA[y][x])
                if ORIG_DATA[y][x].islower():
                    KEYS[ORIG_DATA[y][x]] = (x, y)
                else:
                    GATES[ORIG_DATA[y][x]] = (x, y)
            else:
                n = Node()

            # Check for links
            if ORIG_DATA[y][x - 1] != "#":
                n.links.append((x - 1, y))
            if ORIG_DATA[y-1][x] != "#":
                n.links.append((x, y-1))
            if ORIG_DATA[y][x + 1] != "#":
                n.links.append((x + 1, y))
            if ORIG_DATA[y+1][x] != "#":
                n.links.append((x, y+1))
            NODES[(x, y)] = n


def eliminate_deadend(node, x, y):
    prev = NODES[node.links[0]]
    prev.links.remove((x, y))
    if len(prev.links) == 1 and prev.content is None:
        eliminate_deadend(prev, *node.links[0])
    elif len(curr.links) == 1 and prev.content and prev.content.isupper():
        del GATES[prev.content]
        eliminate_deadend(prev, *node.links[0])
    del NODES[(x, y)]

# Second pass to remove any dead-ends
for y in range(len(ORIG_DATA)):
    for x in range(len(ORIG_DATA[0])):
        if (x, y) in NODES:
            curr = NODES[(x, y)]
            if len(curr.links) == 1 and curr.content is None:
                # Is a dead - end
                eliminate_deadend(curr, x, y)
            elif len(curr.links) == 1 and curr.content and curr.content.isupper():
                # Is a dead - end lock so can be deleted
                eliminate_deadend(curr, x, y)

for y in range(len(ORIG_DATA)):
    line = []
    for x in range(len(ORIG_DATA[0])):
        if (x, y) in NODES:
            curr = NODES[(x, y)]
            if curr.content:
                line.append(curr.content)
            else:
                line.append(".")
        else:
            line.append("#")
    print("".join(line))

print(START)

# Part 2 = 2138
# Solve each robot individually by assuming all the keys for the other robots
# have been collected.


def find_keys(start):
    visit_stack = [(start[0], start[1])]
    seen = set()
    keys = set()
    while visit_stack:
        current = visit_stack.pop(0)
        node = NODES[current]
        if current in seen:
            continue
        seen.add(current)
        if node.content and node.content.islower():
            keys.add(node.content)
        for n in node.links:
            visit_stack.append(n)
    return keys

keys_for_robots = []

for s in START:
    keys_for_robots.append(find_keys(s))

print(keys_for_robots)


def solve_for_one_robot(start, other_keys):
    visit_stack = [((start[0], start[1]), 0, other_keys)]
    seen = set()
    len_keys = 0

    # Avoid back-tracking and repeated visits to the same node
    # by stopping if the node and the collected keys are the same.
    while visit_stack:
        node, dist, keys = visit_stack[0]
        current = NODES[node]
        visit_stack.pop(0)
        unique = (node, tuple(sorted(keys)))
        if unique in seen:
            continue
        seen.add(unique)
        if current.content and current.content.isupper() and current.content.lower() not in keys:
            continue
        new_keys = set(keys)
        if current.content and current.content.islower():
            new_keys.add(current.content)
            if len(new_keys) > len_keys:
                len_keys = len(new_keys)
                print(new_keys)
            if len(new_keys) == len(KEYS):
                print("Done: ", dist)
                return dist
        for k in current.links:
            visit_stack.append((k, dist + 1, new_keys))

total_dist = 0
# Robot 1
other_keys = set()
other_keys.update(keys_for_robots[1])
other_keys.update(keys_for_robots[2])
other_keys.update(keys_for_robots[3])
total_dist += solve_for_one_robot(START[0], other_keys)

# Robot 2
other_keys = set()
other_keys.update(keys_for_robots[0])
other_keys.update(keys_for_robots[2])
other_keys.update(keys_for_robots[3])
total_dist += solve_for_one_robot(START[1], other_keys)

# Robot 3
other_keys = set()
other_keys.update(keys_for_robots[0])
other_keys.update(keys_for_robots[1])
other_keys.update(keys_for_robots[3])
total_dist += solve_for_one_robot(START[2], other_keys)

# Robot 4
other_keys = set()
other_keys.update(keys_for_robots[0])
other_keys.update(keys_for_robots[1])
other_keys.update(keys_for_robots[2])
total_dist += solve_for_one_robot(START[3], other_keys)

print(total_dist)