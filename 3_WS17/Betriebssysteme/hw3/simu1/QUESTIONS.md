# Questions 15-Relocation

The program `relocation.py` allows you to see how address translations are
performed in a system with base and bounds registers. See the `README` for
details.

## Warmup

Run with seeds 1, 2, and 3, and compute whether each virtual address generated
by the process is in or out of bounds. If in bounds, compute the translation.

Please answer the questions, by giving the result and - if asked - an
explanation, why you got the result. Write your answers in markdown syntax in
the new file `ANSWERS.md.`

## Questions

1. Run with these flags: `-s 0 -n 10`. What value do you have set `-l` (the
   bounds register) to in order to ensure that all the generated virtual
   addresses are within bounds?

1. Run with these flags: `-s 1 -n 10 -l 100`. What is the maximum value that
   base can be set to, such that the address space still fits into physical
   memory in its entirety (explanation)?

1. Run some of the same problems above, but with larger address spaces (-a) and
   physical memories (-p). How does increasing effect the results (explanation)?

1. For each virtual address, either write down the physical address it
   translates to OR write down that it is an out-of-bounds address (a
   segmentation violation). For this problem, you should assume a simple virtual
   address space of a given size.

   ```text
   ARG phys mem size 16k

   Base-and-Bounds register information:

     Base   : 0x00003952 (decimal 14674)
     Limit  : 1024

   Virtual Address Trace
     VA  0: 0x0000024c (decimal:  588) --> PA or segmentation violation?
     VA  1: 0x000004eb (decimal: 1259) --> PA or segmentation violation?
     VA  2: 0x000002bd (decimal:  701) --> PA or segmentation violation?
     VA  3: 0x000000fa (decimal:  250) --> PA or segmentation violation?
     VA  4: 0x00000486 (decimal: 1158) --> PA or segmentation violation?
     VA  5: 0x00000521 (decimal: 1313) --> PA or segmentation violation?
     VA  6: 0x0000065c (decimal: 1628) --> PA or segmentation violation?
     VA  7: 0x000001a3 (decimal:  419) --> PA or segmentation violation?
     VA  8: 0x0000063a (decimal: 1594) --> PA or segmentation violation?
     VA  9: 0x0000034d (decimal:  845) --> PA or segmentation violation?
    ```
