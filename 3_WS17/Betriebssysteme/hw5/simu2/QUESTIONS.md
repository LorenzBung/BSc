# Questions: 8-Scheduler-MLFQ

This program, **mlfq.py**, allows you to see how the MLFQ scheduler presented in
this chapter behaves. See the README for details.

Run a few randomly-generated problems with just two jobs and two queues; compute
the MLFQ execution trace for each. Make your life easier by limiting the length
of each job and turning off I/Os.

## Questions

1. How would you run the scheduler to reproduce each of the examples in the
   chapter? Give to each figure number of the PDF chapter the corresponding
   simulator call.
1. How would you configure the scheduler parameters to behave just like a
   round-robin scheduler?
1. Craft a workload with two jobs and scheduler parameters so that one job takes
   advantage of the older Rules 4a and 4b (turned on with the -S flag) to game
   the scheduler and obtain 99% of the CPU over a particular time interval.
1. Given a system with a quantum length of 10ms in its highest queue, how often
   would you have to boost jobs back to the highest priority level (with the -B
   flag) in order to guarantee that a single long-running (and
   potentially-starving) job gets at least 5% of the CPU?
1. One question that arises in scheduling is which end of a queue to add a job
   that just finished I/O; the -I flag changes this behavior for this scheduling
   simulator. Play around with some workloads and see if you can see the effect
   of this flag.
