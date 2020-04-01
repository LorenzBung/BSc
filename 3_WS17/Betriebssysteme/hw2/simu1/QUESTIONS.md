# Questions 14-Memory-API

## Overview

In this task, you will gain some familiarity with memory allocation. First,
you’ll write some buggy programs (fun!). Then, you’ll use some tools to help you
find the bugs you inserted. Then, you will realize how awesome these tools are
and use them in the future, thus making yourself more happy and productive.

The first tool you’ll use is **gdb**, the debugger. There is a lot to learn
about this debugger; here we’ll only scratch the surface. Here you can find a
[quick reference gdb][]. Also **ddd** is installed on lab workstations.

The second tool you’ll use is [valgrind][]. This tool helps find memory leaks
and other insidious memory problems in your program.

Please answer the questions, by giving the result and an explanation, why you
got the result.  Write your answers in markdown syntax in the new file
`ANSWERS.md`. Also checkin your C-Files and one Makefile for all or your
C-Files, so that all binaries are build. Do NOT checkin the binaries!

The installed gcc wrapper on you workstations does some optimization to your code
examples, which will not show some of the bugs with an "Segmentation fault". We
have already disabled this behavior by setting:

```text
export hardeningDisable=all
```

in your labshell bsys environment. So, you don't have to do any further steps to
produce unoptimized code with gcc, if you want to.

If you struggle with your own systems about some strange gcc optimization
behavior, maybe it helps to check for gcc-wrapper variables like these.

## Questions

1. First, write a simple program called `null.c` that creates a pointer to an
   integer, sets it to `NULL`, and then tries to dereference it. Compile this
   into an executable called **null**. What happens when you run this program?

1. Next, compile this program with symbol information included (with the `-g`
   flag). Doing so let’s put more information into the executable, enabling the
   debugger to access more useful information about variable names and the like.
   Run the program under the debugger by typing `gdb null` and then, once gdb is
   running, typing `run`. What does gdb show you?

1. Finally, use the valgrind tool on this program. We’ll use the memcheck tool
   that is a part of valgrind to analyze what happens. Run this by typing in the
   following: `valgrind --leak-check=yes ./null`. What happens when you run
   this? Can you interpret the output from the tool?

1. Write a simple program that allocates memory using `malloc()` but forgets to
   free it before exiting. What happens when this program runs? Can you use
   `gdb` to find any problems with it? How about `valgrind` (again with the
   `--leak-check=yes` flag)?

1. Write a program that creates an array of integers of size 100 using
   `malloc()`; then, set `data[100]` to zero. What happens when you run this
   program? What happens when you run this program using `valgrind`? Is the
   program correct?

1. Create a program that allocates an array of integers (as above),frees them,
   and then tries to print the value of one of the elements of the array. Does
   the program run? What happens when you use `valgrind` on it?

1. Now pass a funny value to free (e.g., a pointer in the middle of the array
   you allocated above). What happens? Do you need tools to find this type of
   problem?

[valgrind]: http://valgrind.org/downloads/current.html
[quick reference gdb]: https://web.stanford.edu/class/cs107/gdb_refcard.pdf
