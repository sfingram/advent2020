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

## Day 6

Puzzles are staying pretty straightforward so far.  This one was just a bunch of set operations.  I had some issues with using references instead of cloning when trying to do the fold in the second half.  Still having to bang my head against the compiler due to misunderstanding some fundamentals here and there.  But hey, that's why I'm here doing this.

## Day 7

The first puzzle I solved by creating an inverted index and the second was a simple recursive solution.  I felt like I struggled with the rust compiler quite a bit to finish this challenge.  The creation of nested, variable-length data structures feel like they fit this puzzle very well, but also mean figuring out where I need to clone and use mutable references.  Clippy gave lots of good advice.

## Day 8

Practice is paying off: ran into some borrowing issues and was able to resolve them much quicker than I had a few days ago.  These are "build an interpreter" puzzles.  The second was a little tricky and I think my solution is kind of stupid because it requires "rewinding" the state of visited instructions.  Would like it if I could just "pop" to the previous state.  There's likely a more efficient way to do this, but gotta move on.

## Day 9

Part one got me; I took a wrong turn and spent way too long implementing it.  There's gotta be a better way to do this one than I did, maybe I'll look at some other people's solutions.  I found part two to be much easier.

## Day 10

Part 1 was very simple, but part 2 needed a recurrence relation to solve.  In terms of rust, I was able to use the excellent itertools crate's `group_by` to structure the computation.  By the way, the recurrence needed generates the [tribonacci numbers](https://en.wikipedia.org/wiki/Generalizations_of_Fibonacci_numbers#Tribonacci_numbers), which are pretty interesting!  Great puzzle! 