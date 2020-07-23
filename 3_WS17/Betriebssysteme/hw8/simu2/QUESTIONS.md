# QUESTIONS Synchronisation: CV

This homework lets you explore some real code that uses locks and condition
variables to implement various forms of the producer/consumer queue discussed in
the chapter. You’ll look at the real code, run it in various configurations, and
use it to learn about what works and what doesn’t, as well as other intricacies.

The different versions of the code correspond to different ways to “solve” the
producer/consumer problem. Most are incorrect; one is correct. Read the chapter
to learn more about what the producer/consumer problem is, and what the code
generally does.

The first step is to download the code and type make to build all the variants.
You should see four:

-   `main-one-cv-while.c`: The producer/consumer problem solved with a single
    condition variable.

-   `main-two-cvs-if.c`: Same but with two condition variables and using an if to
    check whether to sleep.

-   `main-two-cvs-while.c`: Same but with two condition variables and while to
    check whether to sleep. This is the correct version.

-   `main-two-cvs-while-extra-unlock.c`: Same but releasing the lock and then
    reacquiring it around the fill and get routines.

It’s also useful to look at `pc-header.h` which contains common code for all of
these different main programs, and the Makefile so as to build the code
properly.

See the `README.md` for details on these programs.

## Questions

Please explain your answers, even if not asked for it. Try to think first about
the question, then run the code. Remember, in the exam you can't run the code,
but have to think about the question and finally answer it correctly.

### Fix Source Code

When you compile the programs on your workstation, you get a lot of warnings
which are turned into errors. You have to correct these errors first. Check, by
which parameters a thread is created, and how the id is casted later on.

0.  Explain the problem, and how you fixed it.

### `main-two-cvs-while.c`

Our first questions focus on `main-two-cvs-while.c` (the working solution).
First, study the code. Do you think you have an understanding of what should
happen when you run the program?

1.  Now run with one producer and one consumer, and have the producer produce a
    few values. Start with a buffer of size 1, and then increase it. How does the
    behavior of the code change when the buffer is larger? (or does it?)

1.  What would you predict num full to be with different buffer sizes (e.g., -m
    10) and different numbers of produced items (e.g., -l 100), when you change
    the consumer sleep string from default (no sleep) to -C 0,0,0,0,0,0,1?

1.  If possible, run the code on different systems (e.g., Mac OS X , Linux, lab
    container and your notebook). Do you see different behavior across these
    systems?

1.  Let’s look at some timings of different runs. How long do you think the
    following execution, with one producer, three consumers, a single-entry
    shared buffer, and each consumer pausing at point c3 for a second, will take?

      prompt> ./main-two-cvs-while -p 1 -c 3 -m 1 -C
              0,0,0,1,0,0,0:0,0,0,1,0,0,0:0,0,0,1,0,0,0 -l 10 -v -t

1.  Now change the size of the shared buffer to 3(-m3). Will this make any
    difference in the total time?

1.  Now change the location of the sleep to c6 (this models a consumer taking
    something off the queue and then doing something with it for a while), again
    using a single-entry buffer. What time do you predict in this case?

    ```text
     prompt> ./main-two-cvs-while -p 1 -c 3 -m 1 -C
             0,0,0,0,0,0,1:0,0,0,0,0,0,1:0,0,0,0,0,0,1 -l 10 -v -t
    ```

1.  Finally, change the buffer size to 3 again (-m 3). What time do you predict
    now?

### `main-one-cv-while.c`

1.  Now let’s look at `main-one-cv-while.c`. Can you configure a sleep string,
    assuming a single producer, one consumer, and a buffer of size 1, to cause a
    problem with this code? Explain why this happens!

1.  Now change the number of consumers to two. Can you construct sleep strings
    for the producer and the consumers so as to cause a problem in the code?
    Explain why this happens!

### `main-two-cvs-if.c`

1.  Now examine `main-two-cvs-if.c`. Can you cause a problem to happen in this
    code? Again consider the case where there is only one consumer, and then the
    case where there is more than one. Explain the problematic code sequence. 

### `main-two-cvs-while-extra-unlock.c`

1.  Finally, examine `main-two-cvs-while-extra-unlock.c`. What problem arises
    when you release the lock before doing a put or a get? Can you reliably cause
    such a problem to happen, given the sleep strings? What bad thing can happen?
