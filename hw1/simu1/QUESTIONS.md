# Questions 4-Process-Run Simulation Part 1

This program, `process-run.py`, allows you to see how process states change as programs run and either use the CPU (e.g., perform an add instruction) or do I/O (e.g., send a request to a disk and wait for it to complete). See the README for details.

Please answer the questions, by giving the result and an explanation, why you got the result. Sometimes it could be helpful, if you compare your result with results of earlier questions. Write your answers in markdown syntax in the new file `ANSWERS.md.`

1. Run the program with the following flags:

 ```text
./process-run.py -l 5:100,5:100.
 ```

 What should the CPU utilization be (e.g., the percent of time the CPU is in use?) Why do you know this? Use the `-c` and `-p` flags to see if you were right.

2. Now run with these flags:

 ```text
./process-run.py -l 4:100,1:0.
 ```

 These flags specify one process with 4 instructions (all to use the CPU), and one that simply issues an I/O and waits for it to be done. How long does it take to complete both processes? Use `-c` and `-p` to find out if you were right.

3. Now switch the order of the processes:

 ```text
./process-run.py -l 1:0,4:100.
 ```

 What happens now? Does switching the order matter? Why? (As always, use `-c` and `-p` to see if you were right)

4. We’ll now explore some of the other flags. One important flag is `-S`, which determines how the system reacts when a process issues an I/O. With the flag set to `SWITCH_ON_END`, the system will NOT switch to another process while one is doing I/O, instead waiting until the process is completely finished. What happens when you run the following two processes, one doing I/O and the other doing CPU work? (`-l 1:0,4:100 -c -S SWITCH_ON_END`)

5. Now, run the same processes, but with the switching behavior set to switch to another process whenever one is WAITING for I/O (`-l 1:0,4:100 -c -S SWITCH ON IO`). What happens now? Use `-c` and `-p` to confirm that you are right.
