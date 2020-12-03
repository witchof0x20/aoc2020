# Advent of Code 2020

All programs take in their input as stdin


# Running
Example for day 1:
```
cargo run --bin day01 < data/01
```

# Notes
## Day 1
### Part 1
Naive algorithm: Iterate over array in 2 nested loops: O(n^2)
Optimizations discussed in part 2
### Part 2: 
Naive algorithm: Iterate over array in 3 nested loops: O(n^3)
Optimizations:
* Iterate i from 0 until n, j from 0 until i, and k from 0 until j
* Sort the array initially (Initial O(n log n) cost)
* Use binary search in the innermost loop: O(n^2 log n)
* * This implies part 1 can be solved in O(n log n)
* Keep track of when the sum becomes too large, as each outer loop increases the number, keeping track of where the sum exceeded 2020 provided an upper bound for future loops

## Day 2
Easy string parsing problem
### Part 1
Optimizations:
* Stop counting if over the maximum value given
### Part 2
Nothing special, easier than part 1

## Day 3
Just an array indexing problem
## Part 1
Optimizations:
* Use modulo indexing instead of generating the list many times (why would you do this)
## Part 2
Just part 1 with a for loop
