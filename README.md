# Devtimer [![Build Status](https://travis-ci.com/ohsayan/devtimer.svg?branch=master)](https://travis-ci.com/ohsayan/devtimer) [![Crates.io](https://img.shields.io/crates/v/devtimer)](https://crates.io/crates/devtimer) [![Crates.io](https://img.shields.io/badge/docs.rs-Docs-blue)](https://docs.rs/devtimer) [![Crates.io](https://img.shields.io/crates/d/devtimer)](https://crates.io/crates/devtimer) [![Crates.io](https://img.shields.io/crates/l/devtimer)](./LICENSE)
The **compact** yet **complete** benchmarking suite for Rust. Period.

# Rationale

I've seen many, _many_ benchmarking tools. However, no one realizes that we need simplicity to simplify development and increase productivity. 
`devtimer` provides a very _compact_ yet _complete_ benchmarking suite for code written in Rust. 
It makes use of the standard library _only_ to provide benchmark operations. 
You can either use it for benchmarking a single operation or you can use it for
running an operation multiple times and finding the min, max and average 
execution times. Since this crate has no external dependencies, it is small, 
fast and does exactly what it claims to. Happy benchmarking!

Need help migrating from an older version? See [the changelog](./CHANGELOG.md).

# Usage

Add this to your `cargo.toml` :

``` toml
devtimer = "*"
```

Then add this line to your source file (i.e `main.rs` or `lib.rs` or where you need to use it):

``` rust
use devtimer::DevTime;
```

# Example usage

## Simple usage

Let's say there are two functions called `very_long_operation()` and `another_op()` that take a very long time to execute. Then we can time it's execution as shown below:

``` rust
fn main() {
    let mut timer = DevTime::new_simple();
    timer.start();
    very_long_operation();
    timer.stop();
    println!("The operation took: {} ns", timer.time_in_nanos().unwrap());
    // You can keep re-using the timer for other operations
    timer.start(); // this resets the timer and starts it again
    another_op();
    timer.stop();
    println!("The operation took: {} secs", timer.time_in_secs().unwrap());
    println!("The operation took: {} milliseconds", timer.time_in_millis().unwrap());
    println!("The operation took: {} microseconds", timer.time_in_micros().unwrap());
    println!("The operation took: {} ns", timer.time_in_nanos().unwrap());

    // With version 1.1.0 and upwards
    timer.start_after(&std::time::Duration::from_secs(2));
    // The timer will start after two seconds
    // Do some huge operation now
    timer.stop();
    println!("The operation took: {} nanoseconds", devtimer.time_in_nanos().unwrap());
}
```

## Example: Benchmarking

``` rust
use devtimer::run_benchmark;
fn main() {
  // We will simulate a long operation by std::thread::sleep()
  // Run 10 iterations for the test
  let bench_result = run_benchmark(10, |_| {
    // Fake a long running operation
    std::thread::sleep(std::time::Duration::from_secs(1);
  });
  bench_result.print_stats();
}
```

#### Advanced Benchmarking

The `run_benchmark()` function also provides a `usize` that can be used, say if you want to get something from an array to do the test. For example:

```rust 
run_benchmark(100, |n| {
  do_action(data_source[n]);
});
```

## Example: Tagged timers

``` rust
use devtimer::DevTime;
fn main() {
  let mut cmplx = DevTime::new_complex();
  // Create a timer with tag `timer-1` 
  cmplx.create_timer("timer-1").unwrap();
  cmplx.start_timer("timer-1").unwrap();
  // Simulate a slow operation
  std::thread::sleep(std::time::Duration::from_secs(1));
  cmplx.stop_timer("timer-1").unwrap();
  
  // Create a timer with tag `cool-timer` 
  cmplx.create_timer("cool-timer").unwrap();
  cmplx.start_timer("cool-timer").unwrap();
  // Simulate a slow operation
  std::thread::sleep(std::time::Duration::from_secs(2));
  cmplx.stop_timer("cool-timer").unwrap();

  // We can output a benchmark in this way
  println!(" `cool-timer` took: {}", cmplx.time_in_micros("cool-timer").unwrap());

  // Or we can iterate through all timers
  for (tname, timer) in cmplx.iter() {
    println!("{} - {} ns", tname, timer.time_in_micros().unwrap());
  }

  // Or we can print results in the default '{timername} - {time} ns' format
  cmplx.print_stats();
}
```

Timing functions available (names are self explanatory):

* `time_in_secs()` -> Returns the number of seconds the operation took
* `time_in_millis()` -> Returns the number of milliseconds the operation took
* `time_in_micros()` -> Returns the number of microseconds the operation took
* `time_in_nanos()` -> Return the number of nanoseconds the operation took

See the full docs [here](https://docs.rs/devtimer).

# License

This project is licensed under the [Apache-2.0 License](./LICENSE). Keep coding and benchmarking!
