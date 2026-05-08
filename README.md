# Concurrent Task Dispatcher in Rust
# Instructions
You will need the dependency rand.  
Which you can get like so,
```
/cargo add rand
```
Then to run the code simply
```
/cd FinalProject
```
then
```
/cargo run
```
Then follow instructions on screen to select which implementation
# Summary
To make 2 implementations of a Concurrent Task Dispatcher in Rust.  
1.  A simple FIFO approach where workers wait till the task in the  
    fron of the queue can be done.
2.  To use logic in the dispatching so workers can be smartly  
    allocated work to maximize cpu usage.

# Specifications
Main Thread: send tasks  
1000 tasks, sends in intervals of 20ms  
70%/30% distribution of IO/CPU tasks  
80%/20% another option  

Worker Pool: receives tasks and works on them  
Max workers: 8  
Max CPU: 100%  
IO Task: 10% CPU consumption, 200ms of work  
CPU Task: 35% CPU consumption, 200ms of work  

Queue Structure:  
In order to send a task you need to check cpu consumption  
and how many workers there are.  

Monitor Thread: logger, recorder  
Recording in 10ms intervals  
Recording of   
    -Current time  
    -CPU consumption  
    -Worker activity  
Saves data into array  
Then print Results  
    -average CPU consumption  
    -average Worker activity  
    -total time spent working  

1) Simulation FIFO: make sure it follows the logic  
2) Optimize: Change queue logic   

