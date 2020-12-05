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
### Part 1
Optimizations:
* Use modulo indexing instead of generating the list many times (why would you do this)
### Part 2
Just part 1 with a for loop

### Day 4
Just parsing.
### Part 1
I used a `scan` iterator and a horrendous `Option<Result<Option<T>, _>>` return type combined with a `filter_map` to iterate over the lines while allowing for less than 1 output per line. `scan`'s accumulator allowed me to build up passports and validate them. `Some(Ok(None))` means a line parsed and either no new passport was generated or one was, and it turned out to be missing fields. In general, `filter_map` worked nicely for this problem
### Part 2
Just another chain onto part 1.

## Day 5
Another iterator chain

### Part 1
Keep track of min and max for part 2. Easy stuff

### Part 2
I use a tightly packed bit array to store seat state. After padding the left and right seats to mark all remaining seats in a row occupied, all I have to do is check for a position where the row is not equal to 0xFF. I can use the `trailing_ones` function to find the position within that row, which probably compiles down to some processor intrinsic.

