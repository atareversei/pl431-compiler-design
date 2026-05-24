# Threads

System threads are a construct of operating system. They have their own stack memory and can access the shared memory. Operating system context switches between them preemptively (meaning we don't have to do anything special to manage them; e.g. yielding control to other threads when the time is up).

```rs
fn hello_thread(i: u32) {
    println!("Hello from the thread {i}");
    i+1 // threads return like regular function
}

fn main() {
    println!("Hello from the main thread");

    let mut thread_handles = Vec::new();
    for i in 0..5 {
        // i lives for the scope of the for loop
        // that is why we use the `move` keyword to move the ownership
        // to the thread that we just created (it gets copied)
        let thread_handle = std::thread::spawn(move || hello_thread(i));
        thread_handles.push(thread_handle);
    }

    // join makes the main thread wait for all the other threads to
    // stop executing
    thread_handles.into_iter().for_each(|h| {
        println!("{}", h.join().unwrap());
    });
}
```

## Pattern: dividing workloads

In this pattern, we divide a task into independent steps and perform them in parallel.

```rs
fn main() {
    const N_THREADS = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let mut thread_handles = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        thread_handles.push(std::thread::spawn(move || {
            my_chunk.iter().sum::<u32>()
        }))
    }

    let mut sum = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap()
    }
    println!("Sum is {sum}")
}
```

## Pattern: builder pattern

```rs
fn my_thread() {
    println!("Hello from a thread named {}",
        thread::current().name().unwrap()
    );
}


fn main() {
    thread::Builder::new()
        .name("Named Thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap();
}
```

## Pattern: scoped threads

```rs
fn main() {
    const N_THREADS = 8;
    let to_add: Vec<usize> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);

    thread::scoped(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let thread_handle = s.spawn(move || {
                chunk.iter().sum::<u32>()
            });
            thread_handles.push(thread_handle);
        }
        thread_handles.into_iter().map(|h| h.join().unwrap()).sum::<u32>()
    })
}
```

## Atomics

Rust checks data races more effectively than the Go language, up to the point that if someone reports the compiler didn't catch a data race scenario in their code, the Rust team would mark it as a bug and fix it.

```rs
static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 1000 {
        let handle = std::thread::spawn(|| {
            for _ in 1000 {
                COUNTER.fetch_add(1, Relaxed);
            }
        })
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", COUNTER.load(Relaxed));
}

```

## Mutexes

```rs
use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = vec::new();

    for _ in range 0..10 {
        let handle = std::thread::spawn(move || {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push();
        });
        handles.push(handle);
    }

    handles.into_iter.for_each(|h| h.join().unwrap());
}

```

## ReadWrite Locks

```rs
static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Ata".to_string()]
}

fn read_line() -> String {
    let mut input = String::new()
    std::io::stdin().read_line(&mut input).unwrap()
    input.trim().to_string()
}


fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users (in a thread)");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a name to add to user list");
        let input = read_line();
        let mut lock = USERS.write().unwrap();
        lock.push(input);
    }
}
```

## Deadlock, Panics, and Poisoning

Rust won't do anything against deadlocking

```rs
fn main() {
    let my_shared = Mutex::new(0);
    let lock = my_shared.lock().unwrap(); // releases the lock when the scope ends
    let lock = my_shared.lock().unwrap(); // deadlocks
}
```

## Parking Threads

```rs
fn parkable_thread(n: u32) {
    loop {
        std::thread::park();
        println!("Thread {n} is unparked")
    }
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        let handler = std::thread::spawn(move || {
            parkable_thread(i);
        })
        threads.push(handler);
    }

    threads[5].thread().unpark();
}
```

## Channels

```rs
use std::sync::mpsc; // multi producer single consumer

enum Command {
    SayHello, Quit
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match Command {
                // do stuff...
            }
        }
    });

    for _ in 0..10 {
        tx.send(Command::SayHello)
    }

    handle.join()
}
```
