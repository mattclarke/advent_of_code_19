with open("input_data.txt") as f:
    input_data = f.read()

input_data = "03036732577212944063491565474664"

ORIG_DATA = [int(c) for c in input_data]
BASE_PATTERN = [0, 1, 0, -1]


def generate_base(times=1):
    num = 0
    offset = 1
    while True:
        for i in range(times - offset):
            yield BASE_PATTERN[num]
        offset = 0
        num += 1
        num %= len(BASE_PATTERN)


def part_1():
    data = ORIG_DATA[:]

    for _ in range(100):
        new_data = []

        for i, _ in enumerate(data):
            accum = 0
            base_gen = generate_base(i + 1)

            for i in range(len(data)):
                accum += data[i] * next(base_gen)
            new_data.append(abs(accum) % 10)

        data = new_data

    print("".join([str(x) for x in data[0:8]]))


# Part 1 = 53296082
# part_1()

data = ORIG_DATA[:]

print(len(data))
offset = int(input_data[0:8])
for _ in range(1):
    new_data = []

    for i, _ in enumerate(data):
        accum = 0
        base_gen = generate_base(i + 1)
        bases = []

        for i in range(len(data)):
            base = next(base_gen)
            bases.append(base)
            accum += data[i] * base
        accum = abs(accum)
        new_data.append(accum % 10)
        print(bases)

    data = new_data

print(data)
