# Advent of Code 2019

All 50 stars gained!

Started using Rust as a learning exercise, but it was taking so long that I switched to Python.
Note: expect to see very bad Rust code ;)

## Day 20
### Part 1
* Basic shortest path calculation, just treat the portals as normal links.
### Part 2
* Have to keep track of the level of recursion we are in.
* Other than that it is another shortest path calculation.
* Uses `heapq` to make sure we avoid recursing too deep by always putting the lowest levels at the front of the queue.

## Day 21
### Part 1
* If there are any holes between robot and D then jump.
### Part 2
* Needs to handle something like `#####.#.#...#####`, code from part 1 ends up on the first island.
* Solution is to look ahead and only jump if the second jump is possible or we can take a step after the first jump.

## Day 22
### Part 1
* Simple enough: can just create functions to do the three actions and do it for the whole deck.
* Can be simplified to just track the card we are interested in (big speed-up)
### Part 2
* Found this really difficult as had no idea where to begin, so needed internet help. 
* Great puzzle though - I learned a lot.
* Easiest solution to understand:
    * The simplified functions from part 1 are of the linear form ax + b give or take the mod stuff.
    * Going through the commands is essentially h(g(f(x)) for however many commands there are.
    * This means for part 1 it is possible to calculate a "total" a and b and just plug in x
    * For part 2, we need to the inverse of the linear functions and to run through the commands backwards.
    * The inverse of ax + b is y/a - b/a.
    * The complication is for the inverse of the "deal" function because it needs an inverse mod function.
        * 2019 * 500 % 10007 = 8800
        * 8800 / 500 = 17.6 (not 2019)
    * For the deal function we keep adding the total number of cards to y until y/a gives a whole number.
        * n * NUM_CARDS * 8800 / 500 = 2019
    * Finally we need to scale a and b by the number of shuffles.
    * Because this is a big number we use "exponential by squaring" to speed it up.
* Alternative method 1:
    * Because the number of cards is prime we can use Fermat's little theorem to calculate the inverse mod.
    * Fermat's little theorem says inv_mod = a ** (m-2) % m if m is prime
    * So one 'deal 500' for part 1:
        * 2019 * 500 % 10007 = 8800
    * Inverse:
        * 8800 * pow(500, 10005, 10007) % 10007 = 2019
* Alternative method 2:
    * Algebra!
    * Take a and b from doing the calculation forwards
    * Scaling up for the number of shuffles:
        * a becomes a ** num_shuffles % num_cards
        * b expands out into a geometric progression which can be solved but also requires inverse mod.
        * Then the linear equation can be inverted to give the final number. Once again it requires an inverse mod.
        * The inverse mods can be done using Fermat's little theorem
        * This is a very elegant and quick way of solving - there is no way I would have stumbled across this.

## Day 23
### Part 1
* Modified IntCode so that it returns when it has a message ready (i.e. output length = 3).
* Created 50 IntCode computers (arrays of memories, indexes and relative bases).
* Each computer gets its own queue.
* Loop through each computer, sending the front of the queue or -1 if empty.
* Continue unitl the address in an output equals 255.

### Part 2
* Pretty straightforward.
* Added a tuple for NAT and its previous y value.
* Added array to keep track of the idle states.
* When all computers idle send the stored value for NAT.
* Exit if the value sent is the same as the previously sent value.

## Day 24
### Part 1
* Simple enough.
* Run the rules and calculate the "hash", if it matches a previous hash then we are done.

### Part 2
* Start from level 0 (the starting data) and recurse inwards.
* Then recurse outwards (only goes up one level)
* Then repeat starting from the new outermost level
* Logically pretty straightforward but I made a mess of the recursion functions, so took a while to debug.

## Day 25
Just play it!
:)