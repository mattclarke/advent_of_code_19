# Advent of Code 2019
https://adventofcode.com/2019

All 50 stars gained!

Started using Rust as a learning exercise, but it was taking so long that I switched to Python.
Note: expect to see very bad Rust code ;)

## Day 1
### Part 1 & 2
* Simple looping and calculation.

## Day 2
### Part 1
* Start building IntCode with add and multiple functionality.
### Part 2
* Add instruction pointer.

## Day 3
### Part 1 & 2
* Calculating Manhattan distances and shortest paths.

## Day 4
### Part 1 & 2
* Simple looping and comparing.

## Day 5
### Part 1
* Added inputs, outputs and parameter modes to IntCode.
### Part 2
* Added jumps, less than and equals.


## Day 6
### Part 1
* Create a graph of the planets based on which orbits which.
* Do a pre-order traversal and sum the depth for each node/planet.
### Part 2
* Find minimum distance between two specific nodes/planets.
* Start with routes to both from the root node (a common node).
* Move down from common node to a new common root until either:
    * The current "root" is no longer valid for both roots.
    * We reach one of the two nodes.
* Calculate the distance.

## Day 7
### Part 1
* Mostly about working with multiple IntCodes.
* Work out all the permutations for the settings then find the max output.
### Part 2
* Adjust to allow feedback.

## Day 8
### Part 1
* Simple loop and counting exercise.
### Part 2
* Start from the top layer and just keep track of the transparent pixels.

## Day 9
### Part 1
* Complete IntCode by implementing relative base and giving it more "memory".
### Part 2
* Just a test of the IntCode, very simple.

## Day 10
### Part 1
* Divided up the board into on x-axis, on y-axis, top-left, top-right, bottom-left and bottom-right.
* Calculate unit-direction for asteroids, e.g. if at 12,4 then unit = 3, 1
* If unit-direction not seen then count it.
* Repeat for all asteroids and find the maximum.
### Part 2
* Rotate round, see what asteroids are visible then zap them.
* On next rotation, a new set of asteroids are visible so zap them.
* And so on.

## Day 11
### Part 1
* Simple move and count
### Part 2
* Similar, but keep track of the squares' colours so it can be "painted" on screen.

## Day 12
### Part 1
* Calculate velocities, move and repeat.
### Part 2
* Needed some help from the internet.
* Group the x properties for all the moons together, same for y and z.
* Run the simulation for a bit:
    * Record the count when the x properties match their initial state.
    * Record the count when the y properties match their initial state.
    * Record the count when the z properties match their initial state.
* Calculate the least common multiplier for the three numbers (I used a web tool for that!).

## Day 13
### Part 1
* Basically checking the IntCode works by counting the blocks.
### Part 2
* Either play the game or run it automatically by having the bat track the ball.

## Day 14
### Part 1
* Back calculate starting from `1 FUEL`.
### Part 2
* A bit brute force: keep making fuel and calculate the average ore cost as we go.
* Once the average stabilises after a long time, we have the answer (my result was one too high! Need to "floor"
something?).
* Is there a quicker way? Yes, part 1 can be sped up using maths rather than an incrementing loop. Then part 2 can be
done using a binary search until we find the total ore closest but not exceeding the available ore.

## Day 15
### Part 1
* Shortest path calculation.
### Part 2
* Create a queue of squares where oxygen reaches.
* For each item in queue, add oxygen to adjacent squares and add those squares to the queue once the queue is empty
(time += 1).
* And repeat until all have oxygen.

## Day 16
### Part 1
* Simple enough: just some looping.
* Note: after halfway the data become 0000...000111...111
### Part 2
* Because the offset is in the second half of the data we can use the fact that it is all 1s.
* Calculate the total for all the values from the offset onwards (max_num).
* As we move along we can subtract, the previous value from max_num to derive the value for the current location.

## Day 17
### Part 1
* Convert the output into a grid and then find the points where there is scaffolding on all four sides.
### Part 2
* The route, at least for my data, was to follow the scaffolding and at intersections always go straight across.
* Initially I calculated A, B and C manually, but then when back to code it up afterwards.

## Day 18
### Part 1
* Spent a lot of time trying to create a graph with just the keys and doors and the distances between them.
* To make it easier, I eliminated all the parts of the puzzle that didn't need visiting.
* Couldn't get it to work though.
* In the end, needed some internet help.
* Don't try to make a graph of keys and doors instead use the maze as is and do a shortest path search.
* Avoid backtracking and repeated visits to the same node by keeping track of the node and keys seen.
* e.g. visited_set.append((node_cords, keys_collected))
* If a node has been visited before (i.e. it is in the visited_set) stop that path.
* Stop when we have all the keys and the number of steps is the answer
### Part 2
* Needed some help:
    * Someone on the net said you could just treat each robot separately by assuming all the other robots keys have been
     collected.
    * Then it is just a case of adding together the  number of steps for all four robots.
    * This gave me the correct answer, but isn't technically correct as one of the examples fails.
    * This is because there is a "lock" between two of the robots which means we get less steps than we should.
* A better solution from the web:
    * For each kay and robot do a breadth first search to get the distances and doors between it and the other keys.
    * Then a shortest path search where places to visit are queued up if we have the appropriate keys.
    * The queue is ordered so the shortest distance is always first (using `heapq`).

## Day 19
### Part 1
* Basic iteration.
### Part 2
* Start roughly in the ball-park to save some time.
* Look for when the difference between the x of start of the 100th line and the end of the 1st line is 100.

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
