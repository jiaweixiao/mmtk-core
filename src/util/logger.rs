use log::{self, SetLoggerError};
use env_logger::{Builder, Env, Target};

/// Attempt to init a env_logger for MMTk.
pub fn try_init() -> Result<(), SetLoggerError> {
    let mut builder = Builder::from_env(
        Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    builder.target(Target::Stderr);

    builder.try_init()
}
