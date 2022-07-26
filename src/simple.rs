use {
    crate::traits::TimeDifference,
    std::{thread, time::Instant},
};

#[derive(Debug, PartialEq)]
/// A [`SimpleTimer`] is a timer object that can be used for simple timing operations. The timer can
/// be reused by running [`SimpleTimer::reset`].
///
/// ## Example
/// ```
/// use devtimer::{SimpleTimer, TimeDifferenceExt};
///
/// let mut timer = SimpleTimer::new();
/// timer.start();
/// {
///     // so some long operation
/// }
/// timer.stop();
/// println!("{}", timer.time_in_nanos().unwrap());
/// timer.reset(); // reset and use again
/// ```
pub struct SimpleTimer {
    start: Option<Instant>,
    stop: Option<Instant>,
    name: Box<str>,
}

impl TimeDifference for SimpleTimer {
    fn start(&self) -> Option<Instant> {
        self.start
    }
    fn stop(&self) -> Option<Instant> {
        self.stop
    }
}

impl Default for SimpleTimer {
    fn default() -> Self {
        SimpleTimer::new()
    }
}

#[inline(always)]
fn now() -> Option<Instant> {
    Some(Instant::now())
}

impl SimpleTimer {
    fn _new(name: String) -> Self {
        Self {
            start: None,
            stop: None,
            name: name.into_boxed_str(),
        }
    }
    /// Create a new [`SimpleTimer`] witht the default name (either with the name of the thread or
    /// `unnamed-thread`)
    pub fn new() -> Self {
        Self::_new(
            thread::current()
                .name()
                .unwrap_or("unnamed-thread")
                .to_owned(),
        )
    }
    /// Create a new [`SimpleTimer`] with a timer name
    pub fn new_named(name: String) -> Self {
        Self::_new(name)
    }
    /// Resets the timer
    pub fn reset(&mut self) {
        (self.start, self.stop) = (None, None);
    }
}

impl SimpleTimer {
    /// Start the timer
    ///
    /// ## Panics
    ///
    /// This function will panic if the timer was already started
    pub fn start(&mut self) {
        let call_time = now();
        assert!(
            self.start.is_none(),
            "Timer `{}` was already started",
            self.name
        );
        self.start = call_time;
    }
    /// Start the [`SimpleTimer`]. This will return `true` if the timer was never started
    /// and false in other cases
    pub fn start_checked(&mut self) -> bool {
        let call_time = now();
        let not_started = self.start.is_none();
        if not_started {
            self.start = call_time;
        }
        not_started
    }
}

impl SimpleTimer {
    /// Stop the timer
    ///
    /// ## Panics
    ///
    /// This function will panic if the timer has alread been stopped
    pub fn stop(&mut self) {
        let call_time = now();
        assert!(
            self.stop.is_none(),
            "Timer `{}` was already stopped",
            self.name
        );
        self.stop = call_time;
    }
    /// Stop the [`SimpleTimer`]. This will return `true` if the timer was never stopped
    /// and false in other cases
    pub fn stop_checked(&mut self) -> bool {
        let call_time = now();
        let not_stopped = self.stop.is_none();
        if not_stopped {
            self.stop = call_time;
        }
        not_stopped
    }
}
