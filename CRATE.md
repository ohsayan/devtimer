# Devtimer [![Build Status](https://travis-ci.com/sntdevco/devtimer.svg?branch=master)](https://travis-ci.com/sntdevco/devtimer) [![Crates.io](https://img.shields.io/crates/v/devtimer)](https://crates.io/crates/devtimer) [![Crates.io](https://img.shields.io/badge/docs.rs-Docs-blue)](https://docs.rs/devtimer) [![Crates.io](https://img.shields.io/crates/d/devtimer)](https://crates.io/crates/devtimer) [![Crates.io](https://img.shields.io/crates/l/devtimer)](./LICENSE)
Devtimer provides a simple implementation of a timer that can be used to benchmark and time operations, with upto nanosecond level benchmarks.
## Usage
Add this to your `cargo.toml`:
```toml
devtimer = "*"
```
Then add this line to your source file (i.e `main.rs` or `lib.rs` or where you need to use it):
```rust
use devtimer::DevTime;
```
### Example usage
Let's say there are two functions called `very_long_operation()` and `another_op()` that take a very long time to execute. Then we can time it's execution as shown below:
```rust
fn main() {
    let mut timer = DevTime::new();
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
    timer.start_after(std::time::Duration::from_secs(2));
  // The timer will start after two seconds
  // Do some huge operation now
  timer.stop();
  println!("The operation took: {} nanoseconds", devtimer.time_in_nanos().unwrap());
}
```
Timing functions available (names are self explanatory):
- `time_in_secs()` -> Returns the number of seconds the operation took
- `time_in_millis()` -> Returns the number of milliseconds the operation took
- `time_in_micros()` -> Returns the number of microseconds the operation took
- `time_in_nanos()` -> Return the number of nanoseconds the operation took

See the full docs [here](https://docs.rs/devtimer).
