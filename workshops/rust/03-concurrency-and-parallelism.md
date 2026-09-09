# Concurrency and Parallelism

## Tokio

`tokio::spawn` tries to put the task on a different CPU core but that is not guaranteed. We use `yield_now()` to release the CPU to let the other tasks run as well.

```rs
async fn ticker() {
  for i in 0..10 {
    println!("tick {i}");
    tokio::task::yield_now().await();
  }
}

#[tokio::main]
async fn main() {
  let _ = tokio::join!(
    tokio::spawn(ticker()),
    tokio::spawn(ticker()),
  )
}
```

## Blocking Tasks

`spawn_blocking` combined with `std::thread::sleep` helps us pause execution for a certain amount of time in a thread, but if we use the `sleep` inside the thread itself, it will make the whole tokio runtime freeze. We can use `tokio::time::sleep` to make the thread sleep for some amount of time.

```rs
async fn hello_delay(task: u64, delay: u64) {
  println!("Task {task} has started");
  let _ = tokio::task::spawn_blocking(move || {
    std::thread::sleep(dur: Duration::from_millis(time));
  }).await
  tokio::time::sleep(Duration::from_millis(time)).await;
  println!("Task {task} has finished");
}

```
