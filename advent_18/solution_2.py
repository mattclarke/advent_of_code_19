# Part 2
class Node:
    def __init__(self, content=None):
        self.links = []
        self.content = content


with open("input_data_2.txt") as f:
    input_data = f.read()

# Should be 72 but we get 70
# input_data = """
# #############
# #g#f.D#..h#l#
# #F###e#E###.#
# #dCba@#@BcIJ#
# #############
# #nK.L@#@G...#
# #M###N#H###.#
# #o#m..#i#jk.#
# #############"""

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
# Not guaranteed to work (see example above) as there can be situations where
# one robot has to make a step in one direction to enable another one to move
# and then the first robot has to go back the other way which introduces
# "extra" steps which this solution won't pick up.

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

# Example from the web which works for all examples
from heapq import *


KEYS_AND_ROBOTS_COORDS = dict()

Y, X = [-1, 0, 1, 0], [0, 1, 0, -1]
NUM_KEYS = 0
cnt = ord('0')
for r in range(len(ORIG_DATA)):
    for root in range(len(ORIG_DATA[r])):
        if ORIG_DATA[r][root].islower():
            KEYS_AND_ROBOTS_COORDS[ORIG_DATA[r][root]] = (r, root)
            NUM_KEYS += 1
        elif ORIG_DATA[r][root] == '@':
            # Robots are labeled '0', '1', '2' and '3'
            KEYS_AND_ROBOTS_COORDS[chr(cnt)] = (r, root)
            cnt += 1


def breadth_first_search(root):
    QUEUE = [(*root, 0, ())]
    seen, K = set(), dict()
    while QUEUE:
        y, x, dist, doors = QUEUE.pop(0)
        if (y, x) in seen:
            continue
        seen.add((y, x))
        if ORIG_DATA[y][x].islower() and (y, x) != root:
            K[ORIG_DATA[y][x]] = (dist, frozenset(doors))
        for i in range(4):
            dy, dx = y+Y[i], x+X[i]
            if ORIG_DATA[dy][dx] != '#':
                QUEUE.append((dy, dx, dist+1, doors+(ORIG_DATA[dy][dx].lower(),) if ORIG_DATA[dy][dx].isupper() else doors))
    # gives a dict of keys reachable versus their distance away and the keys needed to get there
    return K


GRAPHS = {k: dict() for k in KEYS_AND_ROBOTS_COORDS.keys()}
for root in KEYS_AND_ROBOTS_COORDS.keys():
    for k, destination in breadth_first_search(KEYS_AND_ROBOTS_COORDS[root]).items():
        GRAPHS[root][k] = destination

PRIORITY_QUEUE = [(0, (('0', '1', '2', '3'), frozenset()))]
SEEN = set()
while PRIORITY_QUEUE:
    # Pop off the lowest distance first
    dist, node = heappop(PRIORITY_QUEUE)
    if node in SEEN:
        continue
    SEEN.add(node)
    positions, seen_keys = node
    if len(seen_keys) == NUM_KEYS:
        print(dist)
        break
    for i in range(len(positions)):
        for destination, (dist_to, doors) in GRAPHS[positions[i]].items():
            # Check we have the keys we need and that we haven't been there already
            if len(doors - seen_keys) == 0 and destination not in seen_keys:
                # Replace the current position with the new position, we are only moving one
                # robot at a time.
                new_positions = positions[:i] + (destination,) + positions[i + 1:]
                new_keys = seen_keys | frozenset(destination)
                heappush(PRIORITY_QUEUE, ((dist + dist_to), (new_positions, new_keys)))
