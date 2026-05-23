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
