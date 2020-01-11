with open("input_data.txt") as f:
    input_data = f.read()

# input_data = "03036732577212944063491565474664"

ORIG_DATA = [int(c) for c in input_data]
BASE_PATTERN = [0, 1, 0, -1]


def part_1():
    data = ORIG_DATA[:]

    for _ in range(100):
        new_data = []
        start_index = 0
        index = 0
        length = 1
        base_ = 1

        for _ in data:
            accum = 0

            while index < len(data):
                max_index = index + length if index + length < len(data) else len(data)
                accum += sum(data[index:max_index]) * base_
                base_ *= -1
                index += length * 2
            new_data.append(abs(accum) % 10)
            length += 1
            start_index += 1
            index = start_index
        data = new_data

    print("".join([str(x) for x in data[0:8]]))


# Part 1 = 53296082
part_1()

data = ORIG_DATA[:]

print(len(data))

def part_2():
    data = ORIG_DATA[:] * 10000

    for _ in range(100):
        new_data = []
        start_index = 0
        index = 0
        length = 1
        base_ = 1

        for _ in data:
            accum = 0

            while index < len(data):
                max_index = index + length if index + length < len(data) else len(data)
                accum += sum(data[index:max_index]) * base_
                base_ *= -1
                index += length * 2
            new_data.append(abs(accum) % 10)
            length += 1
            start_index += 1
            index = start_index
        data = new_data
        print(data[0:32])

    print("".join([str(x) for x in data[0:8]]))


offset = int(input_data[0:8])
part_2()