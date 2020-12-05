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

## Day 3

Another simple puzzle, but I did run into my first ownership error in this exercise.  Essentially, the `Vec` of `Vec` of slope structs was getting "moved" when passed in to a closure in the `map`.  This means the memory pointed to by this vec would get dropped after the closure exits and the `Vec` goes out of scope.  This would be fine, except we need to reference that same memory in Part 2.  Anyway, this type of thing is simple to solve if we just use references, which no longer have the side effect of dropping the memory the Vec points to, just the memory associated with the reference.

## Day 4

Data validation, using a lot of regular expressions to do the heavy lifting.  Was going to try to come up with a data structure that takes a string and spits out a data validation function, but just settled on a `match` expression.

## Day 5

The key to this puzzle is to recognize the mapping between the binary space partitioning and bitfields.  Got to use itertools on this one.  I'm a confused about why the `i32` data keeps getting turned into `&i32` or even `&&i32` in the sorted vector and `zip`'ed iterator.  I was able to work around it, but it still feels like magic to me, which I wish it wouldn't.