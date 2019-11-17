//! # Devtimer
//! `devtimer` provides a simple way to time operations using the `DevTime` struct
//! `devtimer` and simply uses the built-in
//! methods available in the standard library to time operations.
//! ##  Example
//!
//! ```
//! extern crate devtimer;
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
use std::time;
/// The `DevTime` struct provides a simple implementation
/// for timing operations using the standard library
pub struct DevTime {
    start: Option<time::Instant>,
    stop: Option<time::Instant>,
}
impl DevTime {
    /// Returns a new instance of the `DevTime` struct
    pub fn new() -> DevTime {
        DevTime {
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
    /// #### Example
    /// ```rust
    /// use devtimer;
    /// use std::time::Duration;
    /// let mut timer = DevTime::new();
    /// timer.start_after(Duration::from_secs(2));
    /// // The timer will automatically start after two seconds
    /// do_some_long_operation();
    /// println!("Time taken: {}", timer.time_in_secs().unwrap());
    /// // The timer can be reused normally again
    /// timer.start(); // this starts the timer instantly
    /// do_another_long_operation();
    /// timer.stop();
    /// println!("Time taken: {}", timer.time_in_secs().unwrap());
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