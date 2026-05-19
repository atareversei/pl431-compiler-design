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
