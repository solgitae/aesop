use crate::error::{AesopError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Unknown,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" | "INFORMATION" => LogLevel::Info,
            "WARN" | "WARNING" => LogLevel::Warn,
            "ERROR" | "ERR" => LogLevel::Error,
            "FATAL" | "CRITICAL" => LogLevel::Fatal,
            _ => LogLevel::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
            LogLevel::Unknown => "UNKNOWN",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!(LogLevel::from_str("error"), LogLevel::Error);
        assert_eq!(LogLevel::from_str("ERROR"), LogLevel::Error);
        assert_eq!(LogLevel::from_str("Error"), LogLevel::Error);
    }

    #[test]
    fn test_from_str_aliases() {
        assert_eq!(LogLevel::from_str("ERR"), LogLevel::Error);
        assert_eq!(LogLevel::from_str("WARNING"), LogLevel::Warn);
        assert_eq!(LogLevel::from_str("INFORMATION"), LogLevel::Info);
        assert_eq!(LogLevel::from_str("CRITICAL"), LogLevel::Fatal);
    }

    #[test]
    fn test_from_str_unknown() {
        assert_eq!(LogLevel::from_str("VERBOSE"), LogLevel::Unknown);
        assert_eq!(LogLevel::from_str(""), LogLevel::Unknown);
    }
}
