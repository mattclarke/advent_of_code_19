from collections import defaultdict

input_data = ""

with open("input_data.txt") as f:
    input_data = f.read()

# input_data = """
# 10 ORE => 10 A
# 1 ORE => 1 B
# 7 A, 1 B => 1 C
# 7 A, 1 C => 1 D
# 7 A, 1 D => 1 E
# 7 A, 1 E => 1 FUEL
# """


YIELDS = {}
RECIPES = {}

lines = input_data.strip().split('\n')
print(lines)

for l in lines:
    _input, _output = l.split('=>')
    num, name = _output.strip().split(' ')
    YIELDS[name] = int(num)
    RECIPES[name] = []
    INGREDIENTS = _input.strip().split(', ')
    for i in INGREDIENTS:
        num, _name = i.strip().split(' ')
        RECIPES[name].append((_name, int(num)))

# print(RECIPES)
# print(YIELDS)


def solve(num_fuel=1):
    leftovers = defaultdict(lambda: 0)
    reactions = defaultdict(lambda: 0)
    reactions["FUEL"] = num_fuel

    ore_count = 0

    while reactions:
        new_reactions = defaultdict(lambda: 0)
        for n,v in reactions.items():
            needed = v

            recipe = RECIPES[n]
            for rn, rv in recipe:
                total = rv * needed
                if rn == "ORE":
                    ore_count += total
                    continue
                if leftovers[rn] > total:
                    leftovers[rn] -= total
                    continue
                else:
                    total -= leftovers[rn]
                    leftovers[rn] = 0

                num_reactions = total // YIELDS[rn]
                b = total % YIELDS[rn]
                if b != 0:
                    num_reactions += 1
                    leftovers[rn] += YIELDS[rn] - b
                new_reactions[rn] += num_reactions
        reactions = new_reactions
    return ore_count

# Part 1 = 579797
print(f"Part 1 total ORE = {solve(1)}")

# Part 2 = 2521844
ORE = 1000000000000
low = 1
high = 1000000000

while True:
    mid = low + (high - low) // 2
    ans = solve(mid)
    if ans > ORE:
        high = mid - 1
    else:
        low = mid + 1
    if abs(high - low) == 1:
        print(f"Part 2 total fuel = {mid}")
        break
