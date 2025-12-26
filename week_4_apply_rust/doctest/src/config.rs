//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use doctest::config::Logging;
//! let config = Logging::new();
//! assert_eq!(config.enabled, false);
//! ```
//!
//! Long-running or environment-dependent example (ignored by doctest):
//! ```ignore
//! use doctest::config::Logging;
//! let mut config = Logging::new();
//! config.enabled = true;
//! // Imagine this starts a background logger and waits for I/O.
//! // This example is ignored to avoid slow or environment-specific runs.
//! ```
//! 
/// Represents the logging level for the application.
/// 
/// # Examples
/// 
/// Creating different log levels:
/// ```
/// use doctest::config::LogLevel;
/// 
/// let debug = LogLevel::Debug;
/// let info = LogLevel::Info;
/// let warn = LogLevel::Warn;
/// let error = LogLevel::Error;
/// ```
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Represents where log output should be written.
/// 
/// # Examples
/// 
/// Writing to standard output:
/// ```
/// use doctest::config::LogOutput;
/// let output = LogOutput::Stdout;
/// ```
/// 
/// Writing to standard error:
/// ```
/// use doctest::config::LogOutput;
/// let output = LogOutput::Stderr;
/// ```
/// 
/// Writing to a file:
/// ```
/// use doctest::config::LogOutput;
/// let output = LogOutput::File(String::from("app.log"));
/// ```
///
/// Example that should compile but not run (e.g., would write to disk):
/// ```no_run
/// use doctest::config::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Info,
///     destination: LogOutput::File(String::from("app.log")),
/// };
/// // If this were real, we might open the file and write:
/// // std::fs::write("app.log", "hello world").unwrap();
/// // Marked no_run so doctest only checks it compiles.
/// ```
///
/// Example that must fail to compile (wrong type for `File` destination):
/// ```compile_fail
/// use doctest::config::LogOutput;
/// // `File` expects a `String`, not an integer.
/// let _bad = LogOutput::File(123);
/// ```
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// 
/// Creating a default configuration:
/// ```
/// use doctest::config::Logging;
/// let config = Logging::new();
/// assert_eq!(config.enabled, false);
/// ```
/// 
/// Creating a custom configuration:
/// ```
/// use doctest::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ 
///     enabled: true, 
///     level: LogLevel::Info, 
///     destination: LogOutput::Stdout 
/// };
/// assert!(config.enabled);
/// ```
/// 
/// Creating a debug logger with file output:
/// ```
/// use doctest::config::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Debug,
///     destination: LogOutput::File(String::from("debug.log")),
/// };
/// assert!(config.enabled);
/// ```
/// 
/// Creating an error-only logger:
/// ```
/// use doctest::config::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Error,
///     destination: LogOutput::Stderr,
/// };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

impl Logging {
    /// Creates a new `Logging` instance with default values.
    /// 
    /// By default, logging is disabled with Info level to Stdout.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// use doctest::config::Logging;
    /// let config = Logging::new();
    /// assert_eq!(config.enabled, false);
    /// ```
    /// 
    /// Modifying the configuration after creation:
    /// ```
    /// use doctest::config::{Logging, LogLevel};
    /// let mut config = Logging::new();
    /// config.enabled = true;
    /// assert!(config.enabled);
    /// ```
    /// 
    /// Changing the log level:
    /// ```
    /// use doctest::config::{Logging, LogLevel};
    /// let mut config = Logging::new();
    /// config.level = LogLevel::Debug;
    /// ```
    ///
    /// Intentional failure example (uncomment locally to see a failing doctest):
    ///
    /// ```ignore
    /// use doctest::config::Logging;
    /// let config = Logging::new();
    /// // Uncomment the next line to see doctest failure reporting:
    /// // assert_eq!(config.enabled, true); // This will panic and fail the doctest.
    /// ```
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}
