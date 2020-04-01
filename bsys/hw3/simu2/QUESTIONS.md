# Questions 16-Segmentation

This program allows you to see how address translations are performed in a
system with segmentation. See the `README` for details.

## Warm-up

First let’s use a tiny address space to translate some addresses. Here’s a
simple set of parameters with a few different random seeds; can you translate
the addresses?

```text
      segmentation.py -a 128 -p 512 -b 0 -l 20 -B 512 -L 20 -s 0
      segmentation.py -a 128 -p 512 -b 0 -l 20 -B 512 -L 20 -s 1
      segmentation.py -a 128 -p 512 -b 0 -l 20 -B 512 -L 20 -s 2
```

## Questions

1. Now, let’s see if we understand this tiny address space we’ve constructed
   (using the parameters from the warm-up above). What is the highest legal
   virtual address in segment 0? What about the lowest legal virtual address in
   segment 1? What are the lowest and highest illegal addresses in this entire
   address space? Finally, how would you run segmentation.py with the -A flag to
   test if you are right?

1. Let’s say we have a tiny 16-byte address space in a 128-byte physical memory.
   What base and bounds would you set up so as to get the simulator to generate
   the following translation results for the specified address stream: valid,
   valid, violation, ..., violation, valid, valid? Assume the following
   parameters:

   ```text
     segmentation.py -a 16 -p 128
         -A 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
         --b0 ? --l0 ? --b1 ? --l1 ?
   ```

1. Assuming we want to generate a problem where roughly 90% of the
   randomly-generated virtual addresses are valid (i.e., not segmentation
   violations). How should you configure the simulator to do so? Which
   parameters are important (Explanation!)?

1. Can you run the simulator such that no virtual addresses are valid? How
   (Explanation)?

1. For each virtual address, either write down the physical address (hex and
   decimal) it translates to OR write down that it is an out-of-bounds address
   (a segmentation violation). For this problem, you should assume a simple
   address space with two segments: the top bit of the virtual address can thus
   be used to check whether the virtual address is in segment 0 (topbit=0) or
   segment 1 (topbit=1). Note that the base/limit pairs given to you grow in
   different directions, depending on the segment, i.e., segment 0 grows in the
   positive direction, whereas segment 1 in the negative.

   ```text
   ARG address space size 364
   ARG phys mem size 756

   Segment register information:

     Segment 0 base  (grows positive) : 0x00000000 (decimal 0)
     Segment 0 limit                  : 64

     Segment 1 base  (grows negative) : 0x00000400 (decimal 1024)
     Segment 1 limit                  : 128

   Virtual Address Trace
     VA  0: 0x0000005c (decimal:   92) --> PA or segmentation violation?
     VA  1: 0x00000011 (decimal:   17) --> PA or segmentation violation?
     VA  2: 0x00000043 (decimal:   67) --> PA or segmentation violation?
     VA  3: 0x00000021 (decimal:   33) --> PA or segmentation violation?
     VA  4: 0x0000006c (decimal:  108) --> PA or segmentation violation?
     VA  5: 0x0000007a (decimal:  122) --> PA or segmentation violation?
     VA  6: 0x00000050 (decimal:   80) --> PA or segmentation violation?
     VA  7: 0x00000037 (decimal:   55) --> PA or segmentation violation?
     VA  8: 0x000000ff (decimal:  255) --> PA or segmentation violation?
     VA  9: 0x000000e9 (decimal:  233) --> PA or segmentation violation?
     VA 10: 0x00000001 (decimal:    1) --> PA or segmentation violation?
     VA 11: 0x0000014c (decimal:  332) --> PA or segmentation violation?
     VA 12: 0x000000b4 (decimal:  180) --> PA or segmentation violation?
     VA 13: 0x000000cf (decimal:  207) --> PA or segmentation violation?
     VA 14: 0x0000012b (decimal:  299) --> PA or segmentation violation?
     VA 15: 0x00000084 (decimal:  132) --> PA or segmentation violation?
   ```

   How did you solve this question?
