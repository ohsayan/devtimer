use std::time::Instant;

pub trait TimeDifference {
    fn start(&self) -> Option<Instant>;
    fn stop(&self) -> Option<Instant>;
}

pub trait TimeDifferenceExt: TimeDifference {
    #[inline(always)]
    fn time_in_nanos(&self) -> Option<u128> {
        match (self.start(), self.stop()) {
            (Some(start), Some(stop)) => Some(stop.duration_since(start).as_nanos()),
            _ => None,
        }
    }

    #[inline(always)]
    fn time_in_micros(&self) -> Option<u128> {
        match (self.start(), self.stop()) {
            (Some(start), Some(stop)) => Some(stop.duration_since(start).as_micros()),
            _ => None,
        }
    }

    #[inline(always)]
    fn time_in_millis(&self) -> Option<u128> {
        match (self.start(), self.stop()) {
            (Some(start), Some(stop)) => Some(stop.duration_since(start).as_millis()),
            _ => None,
        }
    }

    #[inline(always)]
    fn time_in_secs(&self) -> Option<u64> {
        match (self.start(), self.stop()) {
            (Some(start), Some(stop)) => Some(stop.duration_since(start).as_secs()),
            _ => None,
        }
    }
}

impl<T> TimeDifferenceExt for T where T: TimeDifference {}
