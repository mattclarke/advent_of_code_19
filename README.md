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