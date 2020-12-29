//! # Devtimer
//! `devtimer` provides a very **compact** yet **complete** benchmarking suite for code
//! written in Rust. It makes use of the standard library _only_ to provide
//! benchmark operations. You can either use it for benchmarking a single operation or you can
//! use it for running an operation multiple times and finding the min, max and average execution
//! times. Since this crate has no external dependencies, it is small, fast and does exactly what it
//! claims to. Happy benchmarking!
//!
//! ## Examples: `DevTime::new_simple()`
//!
//! ```
//! use devtimer::DevTime;
//! fn main() {
//!     let mut devtime = DevTime::new_simple();
//!     devtime.start();
//!     // Do some long operation
//!     devtime.stop();
//!     println!("The time taken for the operation was: {} nanos", devtime.time_in_nanos().unwrap());
//!     println!("The time taken for the operation was: {} micros", devtime.time_in_micros().unwrap());
//!     println!("The time taken for the operation was: {} millis", devtime.time_in_millis().unwrap());
//!     println!("The time taken for the operation was: {} secs", devtime.time_in_secs().unwrap());
//! }
//! ```
//! ## Example: `devtimer::run_benchmark()`
//!
//! ```
//! use devtimer::run_benchmark;
//! fn main() {
//!     // We will simulate a long operation by std::thread::sleep()
//!     // Run 10 iterations for the test
//!     let bench_result = run_benchmark(10, |_| {
//!         // Fake a long running operation
//!         std::thread::sleep(std::time::Duration::from_secs(1));
//!     });
//!     bench_result.print_stats();
//! }
//! ```
//! ## Examples: `DevTime::new_complex()`
//! ```
//! use devtimer::DevTime;
//! let mut dt = DevTime::new_complex();
//!
//! // Create a new timer tag `pk12`
//! dt.create_timer("pk12").unwrap();
//! dt.start_timer("pk12").unwrap();
//! std::thread::sleep(std::time::Duration::from_micros(12));
//! dt.stop_timer("pk12").unwrap();
//! println!("The operation took: {} us", dt.time_in_micros("pk12").unwrap());
//!
//! // Create a new timer tag `arg2`
//! dt.create_timer("arg2").unwrap();
//! dt.start_timer("arg2").unwrap();
//! std::thread::sleep(std::time::Duration::from_micros(45));
//! dt.stop_timer("arg2").unwrap();
//! println!("The operation took: {} us", dt.time_in_micros("arg2").unwrap());
//!
//! // Use an iterator to iterate over timer and the resulting `SimpleTimer`
//! // to get times in nanos, micros, millis or secs.
//! for (timer, result) in dt.iter() {
//!     println!("Timer: {} - {}", timer, result.time_in_nanos().unwrap());
//! }
//!
//! // Print '{timername} - {time in nanos}'
//! dt.print_results();
//! // Now delete all timers
//! dt.clear_timers();
//! ```
//!
use std::collections::HashMap;
use std::time;
/// The `DevTime` struct provides a simple implementation
/// for benchmarking operations using the standard library.
pub struct DevTime {}

/// The bench struct provides the `benchmark` function that can be used
/// for benchmarking operations using the `bench()` member function
/// Benchmark an operation by running multiple iterations.
/// This function returns a `RunThroughReport` object which can be used to get
/// the benchmark results.
/// ## Example
/// ```
/// use devtimer::run_benchmark;
/// fn main() {
///     // Run 10 iterations
///     let bench_result = run_benchmark(10, |_| {
///         // Fake a slow operation
///         std::thread::sleep(std::time::Duration::from_nanos(10000));
///     });
///     // Now print the benchmark results
///     bench_result.print_stats();
/// }
/// ```
///
pub fn run_benchmark(iters: usize, function: impl Fn(usize)) -> RunThroughReport {
    let mut timer = DevTime::new_simple();
    let mut res = Vec::with_capacity(iters);
    for i in 0..iters {
        println!("Running iter {} ...", i + 1);
        timer.start();
        (function)(i);
        timer.stop();
        res.push(timer.time_in_nanos().unwrap());
    }
    res.sort();
    let realindex = res.len() - 1;
    let fastest = res[0];
    let slowest = res[realindex];
    let sum: u128 = res.into_iter().sum();
    let avg: u128 = sum / (iters as u128);
    RunThroughReport {
        fastest,
        slowest,
        avg,
    }
}

impl DevTime {
    /// Returns a new `SimpleTimer` instance
    pub fn new_simple() -> SimpleTimer {
        SimpleTimer::new()
    }
    /// Returns a new `ComplexTimer` instance
    pub fn new_complex() -> ComplexTimer {
        ComplexTimer::new()
    }
}

/// # Complex Timer
/// A complex timer wraps around a map of timer names and their corresponding
/// `SimpleTimer` instances.
pub struct ComplexTimer {
    /// Map of timers and the corresponding `SimpleTimer`
    timers: HashMap<&'static str, SimpleTimer>,
}

impl ComplexTimer {
    /// Return a new `ComplexTimer` instance
    pub fn new() -> Self {
        ComplexTimer {
            timers: HashMap::new(),
        }
    }
    /// Create a new timer tag. If the timer tag already exists, then this
    /// function returns an error.
    pub fn create_timer(&mut self, timer_name: &'static str) -> Result<(), &'static str> {
        if self.timers.contains_key(timer_name) {
            Err("This timer already exists")
        } else {
            let _ = self.timers.insert(
                timer_name,
                SimpleTimer {
                    start: None,
                    stop: None,
                },
            );
            Ok(())
        }
    }
    /// Start a timer with tag `timer_name`. If this timer tag doesn't exist,
    /// then it returns an error
    pub fn start_timer(&mut self, timer_name: &'static str) -> Result<(), &'static str> {
        match self.timers.get_mut(timer_name) {
            None => return Err("This timer does not exist"),
            Some(x) => {
                x.start = Some(time::Instant::now());
                Ok(())
            }
        }
    }
    /// Stop a timer with tag `timer_name`. If this timer tag doesn't exist,
    /// then it returns an error
    pub fn stop_timer(&mut self, timer_name: &'static str) -> Result<(), &'static str> {
        match self.timers.get_mut(timer_name) {
            None => return Err("This timer does not exist"),
            Some(x) => {
                x.stop = Some(time::Instant::now());
                Ok(())
            }
        }
    }
    /// Get the time in seconds for a timer with tag `timer_name`
    pub fn time_in_secs(&self, timer_name: &'static str) -> Option<u64> {
        match self.timers.get(timer_name) {
            Some(t) => match t.find_diff() {
                Some(diff) => Some(diff.as_secs()),
                None => None,
            },
            None => return None,
        }
    }

    /// Get the time in milliseconds for a timer with tag `timer_name`
    pub fn time_in_millis(&self, timer_name: &'static str) -> Option<u128> {
        match self.timers.get(timer_name) {
            Some(t) => match t.find_diff() {
                Some(diff) => Some(diff.as_millis()),
                None => None,
            },
            None => return None,
        }
    }

    /// Get the time in microseconds for a timer with tag `timer_name`
    pub fn time_in_micros(&self, timer_name: &'static str) -> Option<u128> {
        match self.timers.get(timer_name) {
            Some(t) => match t.find_diff() {
                Some(diff) => Some(diff.as_micros()),
                None => None,
            },
            None => return None,
        }
    }

    /// Get the time in nanoseconds for a timer with tag `timer_name`
    pub fn time_in_nanos(&self, timer_name: &'static str) -> Option<u128> {
        match self.timers.get(timer_name) {
            Some(t) => match t.find_diff() {
                Some(diff) => Some(diff.as_nanos()),
                None => None,
            },
            None => return None,
        }
    }
    /// Delete a timer with tag `timer_name`
    pub fn delete_timer(&mut self, timer_name: &'static str) -> Result<(), &'static str> {
        match self.timers.remove_entry(timer_name) {
            Some(_) => return Ok(()),
            None => return Err("This timer does not exist"),
        }
    }

    /// Delete all set timers
    pub fn clear_timers(&mut self) {
        self.timers.clear();
    }

    /// Print all results in the following format:
    ///
    /// ```log
    /// timerx - 120 ns
    /// timery - 1233 ns
    /// ...
    /// ```
    pub fn print_results(&self) {
        println!("");
        for (k, v) in self.timers.iter() {
            println!("{} - {} ns", k, v.time_in_nanos().unwrap());
        }
    }
    /// Returns an iterator of timer tags and the corresponding `SimpleTimer` instances
    /// # Example
    /// ```
    /// use devtimer::DevTime;
    /// fn main() {
    ///     let mut dt = DevTime::new_complex();
    ///     for (name, timer) in dt.iter() {
    ///         println!("Timer: {} took {} ns", name, timer.time_in_nanos().unwrap());
    ///     }
    /// }
    /// ```
    pub fn iter(&self) -> std::collections::hash_map::Iter<&'static str, SimpleTimer> {
        self.timers.iter()
    }
}

/// The `SimpleTimer` struct holds the start and stop time instances
pub struct SimpleTimer {
    start: Option<time::Instant>,
    stop: Option<time::Instant>,
}
impl SimpleTimer {
    /// Returns a new instance of the `DevTime` struct
    pub fn new() -> Self {
        SimpleTimer {
            start: None,
            stop: None,
        }
    }
    /// Starts a timer on a mutable `DevTime` object
    pub fn start(&mut self) {
        self.start = Some(time::Instant::now());
    }
    /// Stops a timer on a mutable `DevTime` object
    pub fn stop(&mut self) {
        self.stop = Some(time::Instant::now());
    }
    /// Starts a timer after a specified duration
    /// ## Example
    /// ```
    /// use devtimer::DevTime;
    /// use std::time::Duration;
    /// fn main() {
    ///     let mut timer = DevTime::new_simple();
    ///     timer.start_after(&Duration::from_secs(2));
    ///     // The timer will automatically start after two seconds
    ///     // do_some_long_operation();
    ///     timer.stop();
    ///     println!("Time taken: {}", timer.time_in_secs().unwrap());
    ///     // The timer can be reused normally again
    ///     timer.start(); // this starts the timer instantly
    ///     // do_another_long_operation();
    ///     timer.stop();
    ///     println!("Time taken: {}", timer.time_in_secs().unwrap());
    /// }
    /// ```
    /// ### Important Note
    /// This will try to be as precise as possible. However exact precision cannot be guranteed.
    /// As tested on multiple platforms, there are variations in the range of 0 to 10 nanoseconds.
    pub fn start_after(&mut self, dur: &std::time::Duration) {
        std::thread::sleep(*dur);
        self.start = Some(time::Instant::now());
    }
    fn find_diff(&self) -> Option<time::Duration> {
        match self.start {
            Some(start) => match self.stop {
                Some(stop) => {
                    return Some(stop.duration_since(start));
                }
                _ => None,
            },
            _ => None,
        }
    }
    /// Returns an `Option<u128>` with the difference from the
    /// starting time that was created with `start()` and the stop time
    /// that was created with `stop()`. If both the fields exist, then the time
    /// difference is returned in nanoseconds, otherwise `None` is returned
    pub fn time_in_nanos(&self) -> Option<u128> {
        match self.find_diff() {
            Some(duration) => return Some(duration.as_nanos()),
            _ => None,
        }
    }
    /// Returns an `Option<u128>` with the difference from the
    /// starting time that was created with `start()` and the stop time
    /// that was created with `stop()`. If both the fields exist, then the time
    /// difference is returned in microseconds, otherwise `None` is returned
    pub fn time_in_micros(&self) -> Option<u128> {
        match self.find_diff() {
            Some(duration) => return Some(duration.as_micros()),
            _ => None,
        }
    }
    /// Returns an `Option<u128>` with the difference from the
    /// starting time that was created with `start()` and the stop time
    /// that was created with `stop()`. If both the fields exist, then the time
    /// difference is returned in milliseconds, otherwise `None` is returned
    pub fn time_in_millis(&self) -> Option<u128> {
        match self.find_diff() {
            Some(duration) => return Some(duration.as_millis()),
            _ => None,
        }
    }
    /// Returns an `Option<u64>` with the difference from the
    /// starting time that was created with `start()` and the stop time
    /// that was created with `stop()`. If both the fields exist, then the time
    /// difference is returned in seconds, otherwise `None` is returned
    pub fn time_in_secs(&self) -> Option<u64> {
        match self.find_diff() {
            Some(duration) => return Some(duration.as_secs()),
            _ => None,
        }
    }
}
/// The `RunThroughReport` struct provides a benchmark report when calling
/// `DevTime::run_benchmark()`.
/// You can get the slowest, fastest and the average time taken per iteration
/// by the `get_slowest()`, `get_fastest()` and `get_average()` functions
/// respectively.
pub struct RunThroughReport {
    fastest: u128,
    slowest: u128,
    avg: u128,
}
impl RunThroughReport {
    pub fn print_stats(&self) {
        println!("\nSlowest: {} ns", self.slowest);
        println!("Fastest: {} ns", self.fastest);
        println!("Average: {} ns/iter", self.avg);
    }
    pub fn get_fastest(&self) -> u128 {
        self.fastest
    }
    pub fn get_slowest(&self) -> u128 {
        self.slowest
    }
    pub fn get_average(&self) -> u128 {
        self.avg
    }
}

#[test]
fn check_complex_timer_impl() {
    let mut dt = DevTime::new_complex();

    // Create a new timer tag `pk12`
    dt.create_timer("pk12").unwrap();
    dt.start_timer("pk12").unwrap();
    std::thread::sleep(std::time::Duration::from_micros(12));
    dt.stop_timer("pk12").unwrap();
    println!(
        "The operation took: {} us",
        dt.time_in_micros("pk12").unwrap()
    );

    // Create a new timer tag `arg2`
    dt.create_timer("arg2").unwrap();
    dt.start_timer("arg2").unwrap();
    std::thread::sleep(std::time::Duration::from_micros(45));
    dt.stop_timer("arg2").unwrap();
    println!(
        "The operation took: {} us",
        dt.time_in_micros("arg2").unwrap()
    );

    // Use an iterator to iterate over timer and the resulting `SimpleTimer`
    // to get times in nanos, micros, millis or secs.
    for (timer, result) in dt.iter() {
        println!("Timer: {} - {}", timer, result.time_in_nanos().unwrap());
    }

    // Print '{timername} - {time in nanos}'
    dt.print_results();
    // Now delete all timers
    dt.clear_timers();
}

#[test]
fn test_benchmark_impl() {
    use run_benchmark;
    let bench1 = run_benchmark(10, |_| {
        // Simulate a fake slow operation
        std::thread::sleep(time::Duration::from_secs(1));
    });
    // Print the results
    bench1.print_stats();
}

#[test]
fn test_simple_timer_impl() {
    let mut dt = DevTime::new_simple();
    dt.start();
    std::thread::sleep(time::Duration::from_secs(10));
    dt.stop();
    println!("Operation took: {}", dt.time_in_micros().unwrap());
}
