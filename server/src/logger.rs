use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, LoggerHandle, Naming};

use crate::config::LoggerConfig;

pub fn init(config: &LoggerConfig) -> anyhow::Result<LoggerHandle> {
    let mut logger = Logger::try_with_env_or_str(&config.spec)?;
    if let Some(path) = &config.path {
        logger = logger
            .log_to_file(FileSpec::try_from(path)?.suppress_timestamp())
            .rotate(
                Criterion::Age(Age::Day),
                Naming::Timestamps,
                Cleanup::KeepLogFiles(config.keep_log_for_days),
            )
            .append()
    }
    let handle = logger
        .duplicate_to_stdout(if config.duplicate_to_stdout {
            Duplicate::All
        } else {
            Duplicate::None
        })
        .use_utc()
        .format(flexi_logger::colored_detailed_format)
        .start()?;

    Ok(handle)
}
