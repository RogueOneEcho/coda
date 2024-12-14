use rogue_logging::{Logger, TimeFormat, Verbosity};
use std::sync::Arc;
use std::time::SystemTime;

pub fn init_logging() {
    let logger = Logger {
        enabled_threshold: Verbosity::Debug,
        time_format: TimeFormat::Utc,
        start: SystemTime::now(),
        package_name: "rogue_coda".to_owned(),
    };
    let arc = Arc::new(logger);
    Logger::init(arc.clone());
}
