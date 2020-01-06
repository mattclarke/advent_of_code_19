input_data = ""

with open("input_data.txt") as f:
    input_data = f.read()


YIELDS = {}
RECIPES = {}

lines = input_data.split('\n')

for l in lines:
    _input, _output = l.split('=>')
    num, name = _output.strip().split(' ')
    YIELDS[name] = int(num)
    RECIPES[name] = []
    INGREDIENTS = _input.strip().split(', ')
    for i in INGREDIENTS:
        num, _name = i.strip().split(' ')
        RECIPES[name].append((_name, int(num)))

TOTAL_ORE = 1_000_000_000_000  # For part 2

def create_fuel(continous):
    total_ore_used = 0
    fuel_produced = 0
    leftovers = {}

    while True:
        ingredients = RECIPES["FUEL"][:]

        while len(ingredients) > 0:
            name, amount_needed = ingredients.pop(0)
            recipe = RECIPES[name]
            amount_total = leftovers.get(name, 0)
            leftovers[name] = 0

            while amount_total < amount_needed:
                if recipe[0][0] == "ORE":
                    total_ore_used += recipe[0][1]
                else:
                    for r in recipe:
                        ingredients.append(r)
                amount_total += YIELDS[name]
            leftovers[name] = amount_total - amount_needed

        if not continous:
            break

        if total_ore_used > TOTAL_ORE:
            # Takes hours to get here
            break
        fuel_produced += 1

        # Create an estimate
        if fuel_produced % 100 == 0:
            ore_per_fuel = total_ore_used // fuel_produced
            estimate = TOTAL_ORE // ore_per_fuel
            print(f"Estimate = {estimate}")

    return total_ore_used, fuel_produced

# Part 1 = 579797
ore_used, _ = create_fuel(False)
print(f"Part 1 total ORE = {ore_used}")

# Part 2 = 2521844
ore_used, fuel_produced = create_fuel(True)
print(f"Part 2 total ORE = {ore_used}, FUEL = {fuel_produced}")

