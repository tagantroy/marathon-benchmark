use std::path::PathBuf;
use std::time::Instant;
use uuid::Uuid;

pub struct RunConfig {
    pub uuid: Uuid,
    pub working_dir: PathBuf,
    start: Instant,
}

impl RunConfig {
    pub fn new(uuid: Uuid, working_dir: PathBuf, start: Instant) -> Self {
        RunConfig {
            uuid,
            working_dir,
            start,
        }
    }
}
