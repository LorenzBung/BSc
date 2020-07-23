# Questions 20-paging-multilevel-translate

This simulator, paging-policy.py, allows you to play around with different page-replacement policies. See the README for details.

## Warmup

Generate random addresses with the following arguments: -s 0 -n 10, -s 1 -n 10, and -s 2 -n 10. Change the policy from FIFO,to LRU, to OPT. Compute whether each access in said address traces are hits or misses.

## Questions

1. For a cache of size 5, generate worst-case address reference streams for each of the following policies: FIFO, LRU, and MRU (worst-case reference streams cause the most misses possible. For the worst case reference streams, how much bigger of a cache is needed to improve performance dramatically and approach OPT?

1. Generate a random trace (use python or c or rust (single file, no project).
    1. How would you expect the different policies to perform on such a trace?
1. Now generate a trace with some locality.
    1. How can you generate such a trace?
    1. How does LRU perform on it?
    1. How much better than RAND is LRU?
    1. How does CLOCK do? How about CLOCK with different numbers of clock bits?