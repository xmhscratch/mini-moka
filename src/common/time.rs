use chrono::{Duration,DateTime,Utc};

pub(crate) mod clock;

pub(crate) use clock::Clock;

/// a wrapper type over Instant to force checked additions and prevent
/// unintentional overflow. The type preserve the Copy semantics for the wrapped
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub(crate) struct Instant(clock::Instant);

impl Instant {
    pub(crate) fn new(timestamp: DateTime<Utc>) -> Instant {
        Instant(timestamp)
    } 

    pub(crate) fn now() -> Instant {
        Instant(Utc::now())
    }
}

pub(crate) trait CheckedTimeOps {
    fn checked_add(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized;
}

impl CheckedTimeOps for Instant {
    fn checked_add(&self, duration: Duration) -> Option<Instant> {
        self.0.checked_add_signed(duration).map(Instant)
    }
}
