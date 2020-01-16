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
        prev = 0

        for _ in data:
            accum = 0
            if index < len(data) // 2:
                while index < len(data):
                    max_index = index + length if index + length < len(data) else len(data)
                    accum += sum(data[index:max_index]) * base_
                    base_ *= -1
                    index += length * 2
            elif index == len(data) // 2:
                accum += sum(data[index:])
            else:
                # After we reach halfway it is just 1s
                # So we can short circuit it a bit
                accum = prev - data[index-1]
            prev = accum
            new_data.append(abs(accum) % 10)
            length += 1
            start_index += 1
            index = start_index
        data = new_data

    print("".join([str(x) for x in data[0:8]]))


# Part 1 = 53296082
part_1()


def part_2_quicker():
    # Because the offset is in the 2nd half of the data
    # we are at a point where the mask is 000...000111...111
    # This means we can throw a lot of data away
    data = ORIG_DATA[:] * 10000
    offset = int(input_data[0:7])
    print("Offset", offset)

    for _ in range(100):
        new_data = [0] * len(data)
        max_num = sum(data[offset:])
        new_data[offset] = max_num % 10
        for i in range(offset + 1, len(data)):
            max_num -= data[i - 1]
            new_data[i] = max_num % 10
            
        data = new_data
    print("".join([str(x) for x in data[offset:offset+8]]))


# Part 2 = 43310035
part_2_quicker()