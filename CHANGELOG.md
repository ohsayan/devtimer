# Changelog
All changes in this project will be noted in this file.

## Version 3.0.1 [2020-06-15]
> No breaking changes in this release

Fixes issue #2 where the average was calculated incorrectly.

## Version 3.0.0 [2020-05-21]
> This release introduces breaking changes!

- This release splits timer objects into two kinds: `SimpleTimer` and `ComplexTimer`.
    `SimpleTimers` keep things simple: use `.start()` and `.stop()` to benchmark
different operations, however you have to use the results of one benchmark
immediately. This is ideal when you're benchmarking an operation and then printing
or storing the results right away. However, if you want a timername/timer
way of benchmarking, then `ComplexTimer` is for you. 
- Also, the `run_through()`
feature has now been deprecated and instead, you can use the function `run_benchmark()`
which does the same thing. The function was moved out since it didn't rely on a 
specific instance of a `SimpleTimer` (or previously `DevTime`) object.

### Upgrading existing code
- For any code that used `DevTime::new()` in versions prior to `3.0.0`, simply change it to `DevTime::new_simple()`.
- For code that used the `run_through` feature introduced in `2.0.0` can continue to do so, but I recommend 
upgrading to the `run_benchmark` function like shown below:
```rust
use devtimer::run_benchmark;
fn main() {
    let result_of_sleep = run_benchmark(10, || {
        // Fake a slow operation
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
    result_of_sleep.print_stats();
}
```

## Version 2.0.0 [2020-05-09]
> No breaking changes in this release

This release adds the new `run_through()` feature. This new feature completes the gap that was left
in building the _complete_ benchmarking suite for Rust. A single benchmark doesn't say much, so
it is far better to run the benchmark over and over again to see how it perform on average.
The `run_through()` function does exactly that. It accepts the number of iterations as an `usize`
and the benchmark code, either as a closure (`|| {}`) or a function directly. So now that you've
got a complete benchmarking suite, why not benchmark some code?

## Version 1.1.2 [2019-11-27]
> No breaking changes in this release

Fixed minor issues with the documentation which were slightly misleading
## Version 1.1.1 [2019-11-17]
> This release introduces breaking changes!

Take a reference to `std::time::duration` instead of taking it's ownership.
#### Upgrading exisiting code
If you had done something like `timer.start_after(std::time::Duration::from_secs(2))`, all you need to do is add a borrow like shown below:
```rust
timer.start_after(&std::time::Duration::from_secs(2))
```
#### Reason for change
If a common variable is used by the developer, i.e it is declared as follows:
```rust
let dur = std::time::Duration::from_secs(2);
```
Then giving ownership to `start_after()` will be a problem. That is why, this design change was made. Hence, from now on, all you do is:
```rust
let mut timer = DevTime::new();
timer.start_after(&dur);
```

## Version 1.1.0 [2019-11-16]
> No breaking changes in this release
- This version introduces a new function `start_after()`. This can be used to delay the starting of timer operations

## Version 1.0.1 [2019-11-11]
> No breaking changes in this release
- This fixed some issues with the documentation

## Version 1.0.0 [2019-11-10]
- This is the initial release of the crate