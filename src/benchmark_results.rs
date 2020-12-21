use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionReport {
    duration: Duration,
}

impl ExecutionReport {
    pub fn new(start_time: Instant, end_time: Instant) -> Self {
        ExecutionReport {
            duration: end_time - start_time,
        }
    }
}
