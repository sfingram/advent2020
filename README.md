# Advent of Code 2020

This year I wanted to try out these exercises in Rust.  Last year I
did Go and the previous year was Python.  TBH, I had a lot more fun
with Python than Go, which I found pretty verbose (but faster).

It's been a goal of mine to start building some Rust muscle memory,
and nothing like an unpaid sabbatical and a bunch of exercises to make
it happen.

## Day 1

The hardest part of this exercise was just figuring out how the heck to read in a file and create a list of integers from the lines of the file.

### Part 1

Naive solution: O(N^2) iterate over all distinct pairs

Since the size of the input is small (200 lines), this seems fine to me.  I'm using iterators to get more familiar with them.  They remind me of Python comprehensions and Java Streams.

### Part 2

Again, the size of the input is small enough that just looping over distinct triples is going to be instantaneous.

I had some trouble here keeping track of the unwrapping.

## Day 2

Learned a lot from this one about structs and traits, not to mention regex in Rust.  As far as the puzzle goes, still quite simple in both parts.