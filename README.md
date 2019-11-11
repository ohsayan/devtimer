# Devtimer [![Build Status](https://travis-ci.com/sntdevco/devtimer.svg?branch=master)](https://travis-ci.com/sntdevco/devtimer) [![Crates.io](https://img.shields.io/crates/v/devtimer)](https://crates.io/crates/devtimer) [![Crates.io](https://img.shields.io/badge/docs.rs-Docs-blue)](https://docs.rs/devtimer) [![Crates.io](https://img.shields.io/crates/d/devtimer)](https://crates.io/crates/devtimer) [![Crates.io](https://img.shields.io/crates/l/devtimer)](./LICENSE)
Operation benchmarking and timing library for Rust
### Rationale
I've seen many, _many_ benchmarking tools. However, no one realizes that we need simplicity to simplify development and increase productivity. I recently have been up with a lot of perf-testing and I seem to run into a huge need for timing tools. I was initially using `std::time::Instant::now()`and then using the `duration_since()` to find the difference between the two intervals. That was fine, not that it didn't work, but there was a lot of redundancy. So I made a simple library which is a wrapper around the standard library `time` crate.
### Usage
```rust
extern crate devtimer;
use devtimer::DevTime;
fn main() {
  let mut devtimer = DevTime::new();
  devtimer.start();
  // Do some very long operation...
  devtimer.stop();
  println!("The operation took: {} seconds", devtimer.time_in_secs().unwrap());
  // Now keep re-using the timer
  devtimer.start();
  // Do some really really long operation...
  devtimer.stop();
  println!("The operation took: {} nanoseconds", devtimer.time_in_nanos().unwrap());
}
```
### Why are there no tests?
Well, there would be no possible test that I can think of that'd run uniformly across all systems. If I did something like:
```rust
let mut timer = DevTime::new();
timer.start();
std::thread::sleep(std::time::Duration::from_secs(2));
timer.stop();
assert_eq!(timer.time_in_secs().unwrap(), 2);
```
It can easily fail (and has failed) as system calls can take time and the time for them will differ across every system. This will necessarily pass on all systems, but when compared on a microsecond or nanosecond level, the tests have failed multiple times. Hence I decided to omit all tests from this crate.
### License
This project is licensed under the [Apache-2.0 License](./LICENSE). You can virtually do _anything_ with this crate! Just keep coding and benchmarking
