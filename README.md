# Advent of Code 2019

## Day 23
### Part  1
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
### Part  1
* Simple enough.
* Run the rules and calculate the "hash", if it matches a previous hash then we are done.

### Part 2
* Start from level 0 (the starting data) and recurse inwards.
* Then recurse outwards (only goes up one level)
* Then repeat starting from the new outermost level
* Logically pretty straightforward but I made a mess of the recursion functions, so took a while to debug.