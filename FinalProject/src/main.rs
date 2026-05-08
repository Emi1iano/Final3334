use std::{collections::VecDeque};
use rand::{RngExt};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io;

const MAX_CPU: i32 = 100;
const MAX_WORKERS: i32 = 8;
const CPU_CONSUMPTION: i32 = 35;
const IO_CONSUMPTION: i32 = 10;
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    let parsed: i32;
    loop {
        println!("1: FIFO\n2: Optimized");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        parsed = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number. Try again.");
                continue;
            }
        };
        break;
    }
    match parsed {
        1 => {
            let total_tasks = 1000;
            let percent_io = 70;
            let (tx, rx) = mpsc::channel();
            let rx = Arc::new(Mutex::new(rx));
            let task_pool = Arc::new(TaskPool1::new(rx));
            
            let io_task_count = Arc::new(Mutex::new(0));
            let task_pool_main = Arc::clone(&task_pool);
            let io_task_count_clone = Arc::clone(&io_task_count);
            let main_handle = std::thread::spawn(move || {
                println!("Started Main Thread");
                let mut rand_gen = rand::rng();
                for i in 0..total_tasks {
                    //println!("Task {} created.", i+1);
                    //let mut task_pool_main = task_pool_clone.lock().unwrap();
                    let rand_num = rand_gen.random_range(0..100);
                    
                    if rand_num < percent_io {
                        task_pool_main.add(Task { task: TaskType::IO, id: i });
                        *io_task_count_clone.lock().unwrap()+=1;
                    } else {
                        task_pool_main.add(Task { task: TaskType::CPU, id: i });
                    }
                    
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
            });
            
            let task_pool_clone2 = Arc::clone(&task_pool);
            let worker_handle = std::thread::spawn(move || {
                task_pool_clone2.start();
            });
            
            let monitor_running = Arc::new(AtomicBool::new(true));
            let monitor_cpu = Arc::clone(&task_pool.cpu_consumption);
            let monitor_workers = Arc::clone(&task_pool.worker_count);
            let monitor_tasks_done = Arc::clone(&task_pool.tasks_done);
            let monitor_flag = Arc::clone(&monitor_running);
            let io_task_count_clone1 = Arc::clone(&io_task_count);

            let monitor_handle = std::thread::spawn(move || {
                let mut cpu_total = 0;
                let mut worker_total = 0;
                let mut samples = 0;
                let mut time = 0;
                while monitor_flag.load(Ordering::Relaxed) {
                    let cpu = *monitor_cpu.lock().unwrap();
                    let workers = *monitor_workers.lock().unwrap();

                    cpu_total += cpu;
                    worker_total += workers;
                    samples += 1;

                    println!(
                        "[MONITOR {}ms] Tasks Done: {} CPU Usage: {} | Active Workers: {}",
                        time,
                        *monitor_tasks_done.lock().unwrap(),
                        cpu,
                        workers
                    );
                    time += 10;
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }

                if samples > 0 {
                    let avg_cpu = cpu_total as f64 / samples as f64;
                    let avg_workers = worker_total as f64 / samples as f64;
                    let io_tasks = *io_task_count_clone1.lock().unwrap();

                    println!("\n========== AVERAGES ==========");
                    println!("Average CPU Usage: {:.2}", avg_cpu);
                    println!("Average Active Workers: {:.2}", avg_workers);
                    println!("Total Samples: {}", samples);
                    println!("Total Tasks: {}, IO: {}, CPU: {}", total_tasks, io_tasks, total_tasks - io_tasks);
                }
            });

            main_handle.join().unwrap();
            for _ in 0..MAX_WORKERS{tx.send(TERMINATION_SIGNAL).unwrap();}
            worker_handle.join().unwrap();

            monitor_running.store(false, Ordering::Relaxed);
            monitor_handle.join().unwrap();
        },
        2 => {
            let total_tasks = 1000;
            let percent_io = 70;
            let (tx, rx) = mpsc::channel();
            let rx = Arc::new(Mutex::new(rx));
            let task_pool = Arc::new(TaskPool::new(rx));
            
            let io_task_count = Arc::new(Mutex::new(0));
            let task_pool_main = Arc::clone(&task_pool);
            let io_task_count_clone = Arc::clone(&io_task_count);
            let main_handle = std::thread::spawn(move || {
                println!("Started Main Thread");
                let mut rand_gen = rand::rng();
                for i in 0..total_tasks {
                    //println!("Task {} created.", i+1);
                    //let mut task_pool_main = task_pool_clone.lock().unwrap();
                    let rand_num = rand_gen.random_range(0..100);
                    
                    if rand_num < percent_io {
                        task_pool_main.add(Task { task: TaskType::IO, id: i });
                        *io_task_count_clone.lock().unwrap()+=1;
                    } else {
                        task_pool_main.add(Task { task: TaskType::CPU, id: i });
                    }
                    
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
            });
            
            let task_pool_clone2 = Arc::clone(&task_pool);
            let worker_handle = std::thread::spawn(move || {
                task_pool_clone2.start();
            });
            
            let monitor_running = Arc::new(AtomicBool::new(true));
            let monitor_cpu = Arc::clone(&task_pool.cpu_consumption);
            let monitor_workers = Arc::clone(&task_pool.worker_count);
            let monitor_tasks_done = Arc::clone(&task_pool.tasks_done);
            let monitor_flag = Arc::clone(&monitor_running);
            let io_task_count_clone1 = Arc::clone(&io_task_count);

            let monitor_handle = std::thread::spawn(move || {
                let mut cpu_total = 0;
                let mut worker_total = 0;
                let mut samples = 0;
                let mut time = 0;
                while monitor_flag.load(Ordering::Relaxed) {
                    let cpu = *monitor_cpu.lock().unwrap();
                    let workers = *monitor_workers.lock().unwrap();

                    cpu_total += cpu;
                    worker_total += workers;
                    samples += 1;

                    println!(
                        "[MONITOR {}ms] Tasks Done: {} CPU Usage: {} | Active Workers: {}",
                        time,
                        *monitor_tasks_done.lock().unwrap(),
                        cpu,
                        workers
                    );
                    time += 10;
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }

                if samples > 0 {
                    let avg_cpu = cpu_total as f64 / samples as f64;
                    let avg_workers = worker_total as f64 / samples as f64;
                    let io_tasks = *io_task_count_clone1.lock().unwrap();

                    println!("\n========== AVERAGES ==========");
                    println!("Average CPU Usage: {:.2}", avg_cpu);
                    println!("Average Active Workers: {:.2}", avg_workers);
                    println!("Total Samples: {}", samples);
                    println!("Total Tasks: {}, IO: {}, CPU: {}", total_tasks, io_tasks, total_tasks - io_tasks);
                }
            });

            main_handle.join().unwrap();
            for _ in 0..MAX_WORKERS{tx.send(TERMINATION_SIGNAL).unwrap();}
            worker_handle.join().unwrap();

            monitor_running.store(false, Ordering::Relaxed);
            monitor_handle.join().unwrap();
        }
        _ => {}
    }

    
}
enum TaskType {
    IO,
    CPU,
}
struct Task {
    task: TaskType,
    id: i32,
}

struct TaskPool {
    tasks: Arc<Mutex<VecDeque<Task>>>,
    cpu_consumption: Arc<Mutex<i32>>,
    worker_count: Arc<Mutex<i32>>,
    reciever: Arc<Mutex<mpsc::Receiver<i32>>>,
    tasks_done: Arc<Mutex<i32>>,
}
impl TaskPool {
    fn new(rx: Arc<Mutex<mpsc::Receiver<i32>>>) -> TaskPool {
        TaskPool { tasks: Arc::new(Mutex::new(VecDeque::new())), cpu_consumption: Arc::new(Mutex::new(0)), worker_count: Arc::new(Mutex::new(0)), reciever: rx, tasks_done: Arc::new(Mutex::new(0)) }
    }
    fn add(&self, task: Task) {
        self.tasks.lock().unwrap().push_back(task);
    }
    //Using a simple queue to get through the tasks
    fn start(&self) {
        let mut workers = vec![];
        for i in 0..MAX_WORKERS {
            let task_pool_reciever_clone = Arc::clone(&self.reciever);
            let task_pool_tasks_clone = Arc::clone(&self.tasks);
            let task_pool_worker_count = Arc::clone(&self.worker_count);
            let task_pool_cpu_usage = Arc::clone(&self.cpu_consumption);
            let task_pool_tasks_done_clone = Arc::clone(&self.tasks_done);
            let worker_handle = std::thread::spawn(move || {
                let mut done_queuing = false;
                loop {
                    //will recieve a signal from the main thread for when all tasks have been created
                    if !done_queuing {
                        let value = {
                        let receiver = task_pool_reciever_clone.lock().unwrap();
                        receiver.try_recv()
                    };
                            match value {
                            Ok(num) => {
                                if num == TERMINATION_SIGNAL {
                                    // println!("Worker {} got message.", i);
                                    // println!("Everything has been queued");
                                    done_queuing = true;
                                }
                            }
                            Err(mpsc::TryRecvError::Empty) => {}
                            Err(_) => {
                                println!("Error recieving message from main thread in startFIFO thread.");
                                break;
                            }
                        }
                    }
                    
                    //break if all the tasks have been queued and we have finished all the tasks in the queue
                    if done_queuing && task_pool_tasks_clone.lock().unwrap().is_empty() {
                        println!("Worker {} terminated.", i);
                        break;
                    }
                    //get the task at the front of the queue
                    let aux_task = {
                        task_pool_tasks_clone.lock().unwrap().pop_front()
                    };
                    if let Some(task) = aux_task {
                        //how much cpu do we need
                        let cpu_needed = match task.task {
                            TaskType::CPU => CPU_CONSUMPTION,
                            TaskType::IO => IO_CONSUMPTION,
                        };
                        
                        let mut cpu_usage = task_pool_cpu_usage.lock().unwrap();
                        //check to see if we have the resources
                        if *cpu_usage + cpu_needed <= MAX_CPU {
                            //take up the space
                            *cpu_usage += cpu_needed;
                            *task_pool_worker_count.lock().unwrap() += 1;

                            //afer updating cpu drop cpu usage to other threads can work
                            drop(cpu_usage);

                            //do the work
                            std::thread::sleep(std::time::Duration::from_millis(200));
                            *task_pool_tasks_done_clone.lock().unwrap()+=1;

                            let mut cpu_usage = task_pool_cpu_usage.lock().unwrap();
                            //free up space
                            *cpu_usage -= cpu_needed;
                            *task_pool_worker_count.lock().unwrap() -= 1;
                        } else {
                            drop(cpu_usage);
                            //add back to queue we dont got enought cpu
                            task_pool_tasks_clone.lock().unwrap().push_front(task);
                            std::thread::sleep(std::time::Duration::from_millis((i * 4) as u64));
                        }
                    } else {
                        //wait for tasks
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            });
            workers.push(worker_handle);
        }
        for handle in workers {
            handle.join().unwrap();
        }
    }
}

struct TaskPool1 {
    cpu_tasks: Arc<Mutex<VecDeque<Task>>>,
    io_tasks: Arc<Mutex<VecDeque<Task>>>,
    cpu_consumption: Arc<Mutex<i32>>,
    worker_count: Arc<Mutex<i32>>,
    reciever: Arc<Mutex<mpsc::Receiver<i32>>>,
    tasks_done: Arc<Mutex<i32>>,
}
impl TaskPool1 {
    fn new(rx: Arc<Mutex<mpsc::Receiver<i32>>>) -> TaskPool1 {
        TaskPool1 {
            cpu_tasks: Arc::new(Mutex::new(VecDeque::new())),
            io_tasks: Arc::new(Mutex::new(VecDeque::new())),
            cpu_consumption: Arc::new(Mutex::new(0)),
            worker_count: Arc::new(Mutex::new(0)),
            reciever: rx,
            tasks_done: Arc::new(Mutex::new(0))
        }
    }
    fn add(&self, task: Task) {
        match task.task {
            TaskType::CPU => {
                self.cpu_tasks.lock().unwrap().push_back(task);
            }
            TaskType::IO => {
                self.io_tasks.lock().unwrap().push_back(task);
            }
        }
    }
    //Using some logic to priorize 2 cpu 3 io tasks
    fn start(&self) {
        let mut workers = vec![];

        let active_cpu = Arc::new(Mutex::new(0));
        let active_io = Arc::new(Mutex::new(0));

        for i in 0..MAX_WORKERS {
            let receiver_clone = Arc::clone(&self.reciever);

            let cpu_queue = Arc::clone(&self.cpu_tasks);
            let io_queue = Arc::clone(&self.io_tasks);

            let worker_count = Arc::clone(&self.worker_count);
            let cpu_usage = Arc::clone(&self.cpu_consumption);

            let active_cpu_clone = Arc::clone(&active_cpu);
            let active_io_clone = Arc::clone(&active_io);
            let task_pool_tasks_done_clone = Arc::clone(&self.tasks_done);

            let worker_handle = std::thread::spawn(move || {
                let mut done_queuing = false;

                loop {
                    //recieve the signals from the main thread so we know when to consider stopping
                    if !done_queuing {
                        let value = {
                            let receiver = receiver_clone.lock().unwrap();
                            receiver.try_recv()
                        };

                        match value {
                            Ok(num) => {
                                if num == TERMINATION_SIGNAL {
                                    done_queuing = true;
                                }
                            }
                            Err(mpsc::TryRecvError::Empty) => {}
                            Err(_) => {
                                println!("Receiver error.");
                                break;
                            }
                        }
                    }

                    let cpu_empty = cpu_queue.lock().unwrap().is_empty();
                    let io_empty = io_queue.lock().unwrap().is_empty();
                    
                    //we have exhausted all tasks we should terminate the thread
                    if done_queuing && cpu_empty && io_empty {
                        println!("Worker {} terminated.", i);
                        break;
                    }

                    let cpu_running = *active_cpu_clone.lock().unwrap();
                    let mut selected_task = None;

                    //we prioritize running two cpu tasks by selecting a cpu task
                    if cpu_running < 2 {
                        if let Some(task) = cpu_queue.lock().unwrap().pop_front() {
                            selected_task = Some(task);
                        }
                    }

                    //if we have no cpu tasks available we select io tasks
                    //if we have many io tasks we pause to try to get another cpu task
                    if selected_task.is_none() {
                        let io_running = *active_io_clone.lock().unwrap();

                        if io_running < 3 {
                            if let Some(task) = io_queue.lock().unwrap().pop_front() {
                                selected_task = Some(task);
                            }
                        }
                    }
                    
                    //if we have too many io tasks and still no cpu tasks available
                    //then we load more io tasks to work on
                    if selected_task.is_none() {
                        let cpu_empty = cpu_queue.lock().unwrap().is_empty();
                        let cpu_running = *active_cpu_clone.lock().unwrap();

                        if cpu_empty && cpu_running == 0 {
                            if let Some(task) = io_queue.lock().unwrap().pop_front() {
                                selected_task = Some(task);
                            }
                        }
                    }

                    //working on the task
                    if let Some(task) = selected_task {
                        let cpu_needed = match task.task {
                            TaskType::CPU => CPU_CONSUMPTION,
                            TaskType::IO => IO_CONSUMPTION,
                        };

                        let mut usage = cpu_usage.lock().unwrap();

                        if *usage + cpu_needed <= MAX_CPU {
                            *usage += cpu_needed;
                            *worker_count.lock().unwrap() += 1;

                            match task.task {
                                TaskType::CPU => {
                                    *active_cpu_clone.lock().unwrap() += 1;
                                }
                                TaskType::IO => {
                                    *active_io_clone.lock().unwrap() += 1;
                                }
                            }

                            drop(usage);

                            std::thread::sleep(std::time::Duration::from_millis(200));
                            *task_pool_tasks_done_clone.lock().unwrap()+=1;

                            let mut usage = cpu_usage.lock().unwrap();

                            *usage -= cpu_needed;
                            *worker_count.lock().unwrap() -= 1;

                            match task.task {
                                TaskType::CPU => {
                                    *active_cpu_clone.lock().unwrap() -= 1;
                                }
                                TaskType::IO => {
                                    *active_io_clone.lock().unwrap() -= 1;
                                }
                            }
                        } else {
                            drop(usage);

                            // put task back
                            match task.task {
                                TaskType::CPU => {
                                    cpu_queue.lock().unwrap().push_front(task);
                                }
                                TaskType::IO => {
                                    io_queue.lock().unwrap().push_front(task);
                                }
                            }

                            std::thread::sleep(std::time::Duration::from_millis((i * 4) as u64));
                        }
                    } else {
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            });

            workers.push(worker_handle);
        }

        for handle in workers {
            handle.join().unwrap();
        }
    }
}