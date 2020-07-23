# Questions 7-Scheduler-Intro

This program, **scheduler.py**, allows you to see how different schedulers
perform under scheduling metrics such as response time, turnaround time, and
total wait time. See the README for details.

## Questions

1. Compute the average response time and average turnaround time when running
   three jobs of length 200 with the SJF and FIFO schedulers.
1. Now do the same but with jobs of different lengths: 300, 200, and 100.
1. Now do the same (1.+2.), but also with the RR scheduler and a time-slice of
   1.
1. For what types of workloads does SJF deliver the same turnaround times as
   FIFO?
1. For what types of workloads and quantum lengths does SJF deliver the same
   response times as RR?
1. What happens to response time with SJF as job lengths increase? Can you use
   the simulator to demonstrate the trend?
1. What happens to response time with RR as quantum lengths increase? Can you
   write an equation that gives the average worst-case response time, given N
   jobs?