# Advent of Code 2019

## Day 23
### Part  1
* Modified IntCode so that it returns when it has a message ready (i.e. output length = 3).
* Created 50 IntCode computers (arrays of memories, indexes and relative bases).
* Each computer gets its own queue.
* Loop through each computer, sending the front of the queue or -1 if empty.