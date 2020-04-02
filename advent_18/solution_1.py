from collections import deque

with open("input_data.txt") as f:
    input_data = f.read()

# input_data = """
# #################
# #i.G..c...e..H.p#
# ########.########
# #j.A..b...f..D.o#
# ########@########
# #k.E..a...g..B.n#
# ########.########
# #l.F..d...h..C.m#
# #################"""

ORIG_DATA = []
for row in input_data.split():
    print(row)
    r = [c for c in row if c != "\n"]
    ORIG_DATA.append(r)


class Node:
    def __init__(self, content=None):
        self.links = []
        self.content = content


KEYS = {}
GATES = {}
LOOKUP = {}
START = None

NODES = {}

for y in range(len(ORIG_DATA)):
    for x in range(len(ORIG_DATA[0])):
        if ORIG_DATA[y][x] != "#":
            if ORIG_DATA[y][x] == "@":
                START = (y, x)
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


print(KEYS)
print(GATES)
print(START)
print(len(NODES))


def eliminate_deadend(node, x, y):
    prev = NODES[node.links[0]]
    prev.links.remove((x, y))
    if len(prev.links) == 1 and prev.content is None:
        eliminate_deadend(prev, *node.links[0])
    elif len(curr.links) == 1 and prev.content and prev.content.isupper():
        del GATES[prev.content]
        eliminate_deadend(prev, *node.links[0])
    del NODES[(x, y)]

# Second pass to remove any dead - ends
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

print(len(NODES))
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

print(GATES)
print(len(GATES))

# Part 1 = 5402
visit_stack = [((START[1], START[0]), 0, set())]
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
            break
    for k in current.links:
        visit_stack.append((k, dist + 1, new_keys))

# Second attempt
# Build a graph
# GRAPH = {}
# seen = set()
#
# def _recurse(current, links, dist=0):
#     if current in seen:
#         return
#     seen.add(current)
#     for n in current.links:
#         node = NODES[n]
#         if node.content is None:
#             _recurse(node, links, dist + 1)
#         elif node.content.isalpha() or node.content == "@":
#             if node not in seen:
#                 links.add((node.content, dist + 1))
#
#
# for n, v in NODES.items():
#     if v.content:
#         seen.clear()
#         links = set()
#         _recurse(v, links)
#         GRAPH[v.content] = links
#
# visit_stack = [("@", 0, set())]
# seen = set()
# seen_dist = {}
# len_keys = 0
#
# while visit_stack:
#     node, dist, keys = visit_stack[0]
#     visit_stack.pop(0)
#     unique = (node, tuple(sorted(keys)))
#     if unique not in seen:
#         seen.add(unique)
#         seen_dist[unique] = dist
#     elif dist < seen_dist[unique]:
#         seen_dist[unique] = dist
#     else:
#         continue
#     if node.isupper() and node.lower() not in keys:
#         continue
#     new_keys = set(keys)
#     if node.islower():
#         new_keys.add(node)
#         if len(new_keys) > len_keys:
#             len_keys = len(new_keys)
#             print(new_keys)
#         if len(new_keys) == len(KEYS):
#             print(dist, node)
#             continue
#     for k, d in GRAPH[node]:
#         visit_stack.append((k, dist + d, new_keys))
#
