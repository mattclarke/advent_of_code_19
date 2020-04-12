with open("input_data.txt") as f:
    input_data = f.read()

COMMANDS = []
for row in input_data.split("\n"):
    if row.startswith("deal with increment "):
        COMMANDS.append(("deal", int(row.replace("deal with increment ", ""))))
    elif row.startswith("cut "):
        COMMANDS.append(("cut", int(row.replace("cut ", ""))))
    elif row.startswith("deal into new stack"):
        COMMANDS.append(("stack",0))
    else:
        assert False


def deal_into_new_stack(cards):
    cards.reverse()


def cut(cards, n):
    if n > 0:
        taken = cards[:n]
        return cards[n:] + taken
    if n < 0:
        taken = cards[n:]
        return taken + cards[:n]


def deal(cards, inc):
    new_cards = cards[:]
    i = 0
    while cards:
        c = cards.pop(0)
        new_cards[i] = c
        i += inc
        i %= len(new_cards)
    return new_cards


# Part 1 = 6831
a = [i for i in range(10007)]


def slow(a):
    for n, v in COMMANDS:
        if n == "deal":
            a = deal(a, v)
        elif n == "cut":
            a = cut(a, v)
        elif n == "stack":
            deal_into_new_stack(a)
    return a


a = slow(a)
for i, v in enumerate(a):
    if v == 2019:
        print(f"Result (slow) = {i}")
        break

# Track 2019 only = much quicker
num_cards = 10007


def fast_deal(loc, num_cards, inc):
    return (loc * inc) % num_cards


def fast_cut(loc, num_cards, v):
    # Can be replaced by
    # (loc - v) % num_cards
    if v > 0:
        if v < loc:
            loc = loc - v
        else:
            loc = loc - v + num_cards
    elif v < 0:
        if loc < num_cards + v:
            loc = loc - v
        else:
            loc = loc - v - num_cards
    return loc


def fast_stack(loc, num_cards):
    # Can be replaced by
    # return (-loc - 1) % num_cards
    return num_cards - loc - 1


def run(loc, num_cards):
    for n, v in COMMANDS:
        if n == "deal":
            loc = fast_deal(loc, num_cards, v)
        elif n == "cut":
            loc = fast_cut(loc, num_cards, v)
        elif n == "stack":
            loc = fast_stack(loc, num_cards)
    return loc


print(f"Result (fast) = {run(2019, num_cards)}")


# Part 2
# Needed significant help
# Important points:
# 1) Cards repeat past the limits - like a clock-face, i.e. modulus magic!
#
# 2) All steps are linear (ignoring the %), e.g. fast cut = (loc - v) and can be composed, f(g(h(loc)))
#   so ignoring the modulus we have:
#   fast_cut = loc - v  => y = a * x + b where a = 1 and b = -v
#   fast_deal = loc * inc => y = a * x + b where a = inc and b = 0
#   fast_stack = -loc - 1 =>  y = a * x + b where a = -1 and b = -1
#   If any of the answers outside of (0, NUM_CARDS) then can be fixed with % NUM_CARDS
#
# 3) Given f = a*x+b and g = c*x+d, composition g(f(x)) is c * a * x + c * b + d

def also_fast(num_cards):
    a = 1
    b = 0
    for n, v in COMMANDS:
        if n == "deal":
            a = v * a % num_cards
            b = v * b % num_cards
        elif n == "cut":
            b = b - v % num_cards
        elif n == "stack":
            a = a * -1 % num_cards
            b = (b * -1) - 1 % num_cards
    return a, b


a, b = also_fast(10007)
result = (2019 * a + b) % 10007
print(f"Result (also fast) = {result}")

# Now to do it in reverse:
# 1) Invert the functions
#   fast_cut: = loc - v  => loc + v
#   fast_stack = -loc - 1 =>  the same as it just reverses
#   fast_deal = loc * inc => complicated because we mod the answer if it exceeds (0, NUM_CARDS)
#   2019 * 500 % 10007 = 8800
#   the inverse of y = ax + b is x = y/a - b/a, which normally would mean:
#   x = 8800 / 500 = 17.6 (not 2019)
#   By the power of % we can add NUM_CARDS to y until y / 500 gives an int
#   n * NUM_CARDS * y / 500 = 2019
#
# 2) Run through the inverse commands backwards to get a and b
#
# 3) Scale a and b to the number of times we have to repeat the shuffle (NUM_SHUFFLES)
#   if f(x) is one complete shuffle then we are doing f many times
#   NUM_SHUFFLES is big, so need to use a trick to speed it up:
#   From SICP we have exponential by squaring (see https://en.wikipedia.org/wiki/Exponentiation_by_squaring):
#       x ** n = x * (x * x)  ** (n/2- 1/2) if n is odd
#       x ** n = (x * x)  ** (n/2) if n is even


def reverse_deal_v1(a, v, num_cards):
    while a % v != 0:
        a += num_cards
    return a // v


def backwards_inverse(num_cards, reverse_deal=reverse_deal_v1):
    # Run the inverse commands backwards
    a = 1
    b = 0
    for cmd, v in reversed(COMMANDS):
        if cmd == "deal":
            a = reverse_deal(a, v, num_cards) % num_cards
            b = reverse_deal(b, v, num_cards) % num_cards
        elif cmd == "cut":
            b = (b + v) % num_cards
        elif cmd == "stack":
            a *= -1 % num_cards
            b = (b * -1) - 1 % num_cards
    return a, b


# g(f(x)) => g(ax + b) => cax + cb + d
# For a, this is pure exponential by squaring.
# For b, it is slightly more complicated because of the extra term
def exp_by_squaring_combined(a, b, num_shuffles, num_cards):
    # Use % to stop a and b getting too big as that slows the algorithm down
    a %= num_cards
    b %= num_cards

    if num_shuffles == 0:
        return 1, 0
    elif num_shuffles % 2 == 0:
        return exp_by_squaring_combined(a * a, (a * b + b), num_shuffles // 2, num_cards)
    else:
        c, d = exp_by_squaring_combined(a, b, num_shuffles - 1, num_cards)
        return a * c, a * d + b


# Try with part 1
a, b = backwards_inverse(10007)
result = (6831 * a + b) % 10007
print(f"Part 1 reversed = {result}")

# Part 2 = 81781678911487
NUM_CARDS = 119315717514047
NUM_SHUFFLES = 101741582076661

a, b = backwards_inverse(NUM_CARDS)
a, b = exp_by_squaring_combined(a, b, NUM_SHUFFLES, NUM_CARDS)
result = (2020 * a + b) % NUM_CARDS
print(f"Part 2 = {result}")


# A quicker way to do the reverse deal is to use Fermat's little theorem.
# This can be used because NUM_CARDS is prime.
# Fermat's little theorem says inv_mod = a ** (m-2) % m if m is prime
# So one 'deal 20' for part 1:
#    ans = 2019 * 20 % 10007 = 352
# Inverse:
#    ans = 352 * pow(20, 10005) % 10007 = 2019
def fermat(a, v, num_cards):
    # The third parameter of pow means we are calculating x ** y % z
    # It is significantly more efficient to calculate it during the pow than to do it afterwards.
    inv_mod = pow(v, num_cards - 2, num_cards)
    # Note: The line above is the Fermat bit, the below relates to the specifics of the task.
    # Additional % num_cards just to keep the numbers small (-ish)
    a = a * inv_mod % num_cards
    return a


a, b = backwards_inverse(NUM_CARDS, fermat)
a, b = exp_by_squaring_combined(a, b, NUM_SHUFFLES, NUM_CARDS)
result = (2020 * a + b) % NUM_CARDS
print(f"Part 2 (Fermat) = {result}")

# One last way based on algebra
# For one shuffle, can just rearrange equation.
# y = ax + b => x = (y - b) / a
# but need inverse of a mod num_cards
a, b = also_fast(10007)
result = (2019 * a + b) % 10007
backwards_result = ((result - b) * pow(a, 10007-2)) % 10007
print(f"{backwards_result}")

# With multiple shuffles
# a is simply a ** num_shuffles % num_cards
# b is more complicated, for num_shuffles = 3:
# f(x) = ax + b
# f(f(x)) = a(ax + b) + b = aax + ab + b
# f(f(f(x))) = a(aax + ab + b) + b = aaax + aab + ab + b
# x starts as 1, so this simplifies to:
# b * (1 + a + aa + aaa)
# b * (1 + a ** 1 + a ** 2 + a ** 3)
# This is a geometric progression, so to get the M'th term
#
# Mth = b * (1 - a ** m) / (1-a)
#
# Again, because of the mod stuff we need the inverse mod of (1-a)
a, b = also_fast(NUM_CARDS)
mxa = pow(a, NUM_SHUFFLES, NUM_CARDS)
mxb = (b * (1 - mxa) * pow(1 - a, NUM_CARDS - 2, NUM_CARDS)) % NUM_CARDS
result = ((2020 - mxb) * pow(mxa, NUM_CARDS - 2, NUM_CARDS)) % NUM_CARDS
print(result)
