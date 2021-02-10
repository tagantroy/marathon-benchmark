use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionReport {
    suite_name: String,
    iteration: u32,
    duration: Duration,
}

impl ExecutionReport {
    pub fn new(suite_name: String, iteration: u32, start_time: Instant, end_time: Instant) -> Self {
        ExecutionReport {
            suite_name,
            iteration,
            duration: end_time - start_time,
        }
    }
}
