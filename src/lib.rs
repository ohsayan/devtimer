//! # Devtimer
//! `devtimer` provides a very **compact** yet **complete** benchmarking suite for code
//! written in Rust. It makes use of the standard library _only_ to provide
//! benchmark operations. You can either use it for benchmarking a single operation or you can
//! use it for running an operation multiple times and finding the min, max and average execution
//! times. Since this crate has no external dependencies, it is small, fast and does exactly what it
//! claims to. Happy benchmarking!
//! ##  Simple usage example
//!
//! ```
//! use devtimer::DevTime;
//! fn main() {
//!     let mut devtime = DevTime::new();
//!     devtime.start();
//!     // Do some long operation
//!     devtime.stop();
//!     println!("The time taken for the operation was: {} nanos", devtime.time_in_nanos().unwrap());
//!     println!("The time taken for the operation was: {} micros", devtime.time_in_micros().unwrap());
//!     println!("The time taken for the operation was: {} millis", devtime.time_in_millis().unwrap());
//!     println!("The time taken for the operation was: {} secs", devtime.time_in_secs().unwrap());
//! }
//! ```
//! ## Advanced usage example
//!
//! ```
//! use devtimer::DevTime;
//! fn main() {
//!     let mut dt = DevTime::new();
//!     // We will simulate a long operation by std::thread::sleep()
//!     // Run 10 iterations for the test
//!     let bench_result = dt.run_through(10, || {
//!         // Fake a long running operation
//!         std::thread::sleep(std::time::Duration::from_secs(1));
//!     });
//!     bench_result.print_stats();
//! }
//! ```
//!
use std::collections::HashMap;
use std::time;
/// The `DevTime` struct provides a simple implementation
/// for benchmarking operations using the standard library.
pub struct DevTime {}

impl DevTime {
    pub fn new_simple() -> SimpleTimer {
        SimpleTimer::new()
    }
    pub fn new_complex() -> ComplexTimer {
        ComplexTimer::new()
    }
}

pub struct ComplexTimer {
    timers: HashMap<&'static str, SimpleTimer>,
}

impl ComplexTimer {
    pub fn new() -> Self {
        ComplexTimer {
            timers: HashMap::new(),
        }
    }
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
    pub fn start_timer(&mut self, timer_name: &'static str) -> Result<(), &'static str> {
        match self.timers.get_mut(timer_name) {
            None => return Err("This timer does not exist"),
            Some(x) => {
                x.start = Some(time::Instant::now());
                Ok(())
            }
        }
    }
    pub fn stop_timer(&mut self, timer_name: &'static str) -> Result<(), &'static str> {
        match self.timers.get_mut(timer_name) {
            None => return Err("This timer does not exist"),
            Some(x) => {
                x.stop = Some(time::Instant::now());
                Ok(())
            }
        }
    }
    pub fn time_in_secs(&self, timer_name: &'static str) -> Option<u64> {
        match self.timers.get(timer_name) {
            Some(t) => match t.find_diff() {
                Some(diff) => Some(diff.as_secs()),
                None => None,
            },
            None => return None,
        }
    }

    pub fn time_in_millis(&self, timer_name: &'static str) -> Option<u128> {
        match self.timers.get(timer_name) {
            Some(t) => match t.find_diff() {
                Some(diff) => Some(diff.as_millis()),
                None => None,
            },
            None => return None,
        }
    }

    pub fn time_in_micros(&self, timer_name: &'static str) -> Option<u128> {
        match self.timers.get(timer_name) {
            Some(t) => match t.find_diff() {
                Some(diff) => Some(diff.as_micros()),
                None => None,
            },
            None => return None,
        }
    }

    pub fn time_in_nanos(&self, timer_name: &'static str) -> Option<u128> {
        match self.timers.get(timer_name) {
            Some(t) => match t.find_diff() {
                Some(diff) => Some(diff.as_nanos()),
                None => None,
            },
            None => return None,
        }
    }
    pub fn run_through(
        &mut self,
        timer_name: &'static str,
        iters: usize,
        function: fn() -> (),
    ) -> Result<RunThroughReport, ()> {
        let timer = match self.timers.get_mut(timer_name) {
            Some(t) => t,
            None => return Err(()),
        };
        let mut res = Vec::new();
        for i in 0..iters {
            println!("Running iter {} ...", i + 1);
            timer.start();
            (function)();
            timer.stop();
            res.push(timer.time_in_nanos().unwrap());
        }
        res.sort();
        let realindex = res.len() - 1;
        let fastest = res[0];
        let slowest = res[realindex];
        let mut tot = 0;
        res.into_iter().for_each(|x| {
            tot += x;
        });
        let avg: u128 = tot / (realindex as u128);
        Ok(RunThroughReport {
            fastest,
            slowest,
            avg,
        })
    }
    
    pub fn delete_timer(&mut self, timer_name: &'static str) -> Result<(), &'static str> {
        match self.timers.remove_entry(timer_name) {
            Some(_) => return Ok(()),
            None => return Err("This timer does not exist"),
        }
    }

    pub fn clear_timers(&mut self) {
        self.timers.clear();
    }

    pub fn print_results(&self) {
        println!("");
        for (k, v) in self.timers.iter() {
            println!("{} - {} ns", k, v.time_in_nanos().unwrap());
        }
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<&'static str, SimpleTimer> {
        self.timers.iter()
    }
}

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
    ///     let mut timer = DevTime::new();
    ///     timer.start_after(&Duration::from_secs(2));
    ///     // The timer will automatically start after two seconds
    ///     // do_some_long_operation();
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

    /// Benchmark an operation by running multiple iterations.
    /// This function returns a `RunThroughReport` object which can be used to get
    /// the benchmark results.
    /// ## Example
    /// ```
    /// use devtimer::DevTime;
    /// fn main() {
    ///     let mut dt = DevTime::new();
    ///     // Run 10 iterations
    ///     let bench_result = dt.run_through(10, || {
    ///         // Fake a slow operation
    ///         std::thread::sleep(std::time::Duration::from_nanos(10000));
    ///     });
    ///     // Now print the benchmark results
    ///     bench_result.print_stats();
    /// }
    /// ```
    ///
    pub fn run_through(&mut self, iters: usize, function: fn() -> ()) -> RunThroughReport {
        let mut res = Vec::new();
        for i in 0..iters {
            println!("Running iter {} ...", i + 1);
            self.start();
            (function)();
            self.stop();
            res.push(self.time_in_nanos().unwrap());
        }
        res.sort();
        let realindex = res.len() - 1;
        let fastest = res[0];
        let slowest = res[realindex];
        let mut tot = 0;
        res.into_iter().for_each(|x| {
            tot += x;
        });
        let avg: u128 = tot / (realindex as u128);
        RunThroughReport {
            fastest,
            slowest,
            avg,
        }
    }
}
/// The `RunThroughReport` struct provides a benchmark report when calling
/// `DevTime::run_through()`.
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
    
    dt.create_timer("randomthread").unwrap();
    dt.run_through("randomthread", 10, || {
        std::thread::sleep(std::time::Duration::from_micros(15));
    }).unwrap().print_stats();
    
    dt.create_timer("pk12").unwrap();
    dt.start_timer("pk12").unwrap();
    std::thread::sleep(std::time::Duration::from_micros(12));
    dt.stop_timer("pk12").unwrap();
    println!("The operation took: {} us", dt.time_in_micros("pk12").unwrap());

    dt.create_timer("arg2").unwrap();
    dt.start_timer("arg2").unwrap();
    std::thread::sleep(std::time::Duration::from_micros(45));
    dt.stop_timer("arg2").unwrap();
    println!("The operation took: {} us", dt.time_in_micros("arg2").unwrap());

    for (timer, result) in dt.iter() {
        println!("Timer: {} - {}", timer, result.time_in_nanos().unwrap());
    }

    dt.print_results();
    // Now delete all timers
    dt.clear_timers();
}