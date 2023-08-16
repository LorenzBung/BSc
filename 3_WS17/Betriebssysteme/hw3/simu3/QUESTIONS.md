# Questions 17-FreeSpace-Management

The program, `malloc.py`, lets you explore the behavior of a simple free-space
allocator as described in the chapter. See the README for details of its basic
operation.

## Warm-up

First run with the flags `-n 10 -H 0 -p BEST -s 0` to generate a few random
allocations and frees. Can you predict what *alloc()*/*free()* will return? Can
you guess the state of the free list after each request? What do you notice
about the free list over time?

## Questions

1. How are the results different when using a WORST fit policy to search the
   free list (`-p WORST`) from warm-up? What changes (Explanation)?

1. What about when using FIRST fit (`-p FIRST`)? What speeds up when you use
   first fit (Explanation)?

1. For the above questions, how the list is kept ordered can affect the time it
   takes to find a free location for some of the policies. Use the different
   free list orderings (`-l ADDRSORT, -l SIZESORT+, -l SIZESORT-`) to see how
   the policies and the list orderings interact. What are your observations?

1. Coalescing of a free list can be quite important. Increase the number of
   random allocations (say to `-n 1000`). What happens to larger allocation
   requests over time? Run with and without coalescing (i.e., without and with
   the `-C` flag). What differences in outcome do you see? How big is the free
   list over time in each case? Does the ordering of the list matter in this
   case? Comment your results!

1. What happens when you change the percent allocated fraction `-P` to higher
   than 50? What happens to allocations as it nears 100? What about as it nears
   0? Please give an explanation to your observations.

1. What kind of specific requests can you make to generate a highly-fragmented
   free space? Use the `-A` flag to create fragmented free lists, and see how
   different policies and options change the organization of the free list.
   Explain your results.
