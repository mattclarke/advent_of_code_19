with open("input_data.txt") as f:
    input_data = f.readlines()

ORIG_DATA = []
for row in input_data:
    r = [c for c in row if c != "\n"]
    ORIG_DATA.append(r)

print(ORIG_DATA)
