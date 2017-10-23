
#Antworten - Simulation 2

### SWITCH_ON_IO

>process-run.py -l 3:0,5:100,5:100,5:100 -S SWITCH_ON_IO -I IO_RUN_LATER -c -p
>Time     PID: 0     PID: 1     PID: 2     PID: 3        CPU        IOs 
>  1      RUN:io      READY      READY      READY          1            
>  2     WAITING    RUN:cpu      READY      READY          1          1 
>  3     WAITING    RUN:cpu      READY      READY          1          1 
>  4     WAITING    RUN:cpu      READY      READY          1          1 
>  5     WAITING    RUN:cpu      READY      READY          1          1 
>  6*      READY    RUN:cpu      READY      READY          1            
>  7       READY       DONE    RUN:cpu      READY          1            
>  8       READY       DONE    RUN:cpu      READY          1            
>  9       READY       DONE    RUN:cpu      READY          1            
> 10       READY       DONE    RUN:cpu      READY          1            
> 11       READY       DONE    RUN:cpu      READY          1            
> 12       READY       DONE       DONE    RUN:cpu          1            
> 13       READY       DONE       DONE    RUN:cpu          1            
> 14       READY       DONE       DONE    RUN:cpu          1            
> 15       READY       DONE       DONE    RUN:cpu          1            
> 16       READY       DONE       DONE    RUN:cpu          1            
> 17      RUN:io       DONE       DONE       DONE          1            
> 18     WAITING       DONE       DONE       DONE                     1 
> 19     WAITING       DONE       DONE       DONE                     1 
> 20     WAITING       DONE       DONE       DONE                     1 
> 21     WAITING       DONE       DONE       DONE                     1 
> 22*     RUN:io       DONE       DONE       DONE          1            
> 23     WAITING       DONE       DONE       DONE                     1 
> 24     WAITING       DONE       DONE       DONE                     1 
> 25     WAITING       DONE       DONE       DONE                     1 
> 26     WAITING       DONE       DONE       DONE                     1 
> 27*       DONE       DONE       DONE       DONE                       

>Stats: Total Time 27
>Stats: CPU Busy 18 (66.67%)
>Stats: IO Busy  12 (44.44%)



process-run.py -l 3:0,5:100,5:100,5:100 -S SWITCH_ON_IO -I IO_RUN_IMMEDIATE -c -p
Time     PID: 0     PID: 1     PID: 2     PID: 3        CPU        IOs 
  1      RUN:io      READY      READY      READY          1            
  2     WAITING    RUN:cpu      READY      READY          1          1 
  3     WAITING    RUN:cpu      READY      READY          1          1 
  4     WAITING    RUN:cpu      READY      READY          1          1 
  5     WAITING    RUN:cpu      READY      READY          1          1 
  6*     RUN:io      READY      READY      READY          1            
  7     WAITING    RUN:cpu      READY      READY          1          1 
  8     WAITING       DONE    RUN:cpu      READY          1          1 
  9     WAITING       DONE    RUN:cpu      READY          1          1 
 10     WAITING       DONE    RUN:cpu      READY          1          1 
 11*     RUN:io       DONE      READY      READY          1            
 12     WAITING       DONE    RUN:cpu      READY          1          1 
 13     WAITING       DONE    RUN:cpu      READY          1          1 
 14     WAITING       DONE       DONE    RUN:cpu          1          1 
 15     WAITING       DONE       DONE    RUN:cpu          1          1 
 16*       DONE       DONE       DONE    RUN:cpu          1            
 17        DONE       DONE       DONE    RUN:cpu          1            
 18        DONE       DONE       DONE    RUN:cpu          1            

Stats: Total Time 18
Stats: CPU Busy 18 (100.00%)
Stats: IO Busy  12 (66.67%)

