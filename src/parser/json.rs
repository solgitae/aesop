use std::{collections::HashMap, path::PathBuf};

use serde_json::Value;

use crate::{
    error::{AesopError, Result},
    parser::level::LogLevel,
};

pub struct ParsedLine {
    pub raw: String,
    pub level: LogLevel,
    pub message: Option<String>,
    pub timestamp: Option<String>,
    pub fields: HashMap<String, Value>,
}

impl ParsedLine {
    pub fn parse(raw: &str, path: &PathBuf) -> Result<Self> {
        let value: Value = serde_json::from_str(raw.trim())
            .map_err(|_| AesopError::NotJsonFormat { path: path.clone() })?;
        let obj = value
            .as_object()
            .ok_or_else(|| AesopError::NotJsonFormat { path: path.clone() })?;

        let level = extract_level(obj);
        let message = extract_message(obj);
        let timestamp = extract_timestamp(obj);

        let fields: HashMap<String, Value> =
            obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        Ok(ParsedLine {
            raw: raw.to_string(),
            level,
            message,
            timestamp,
            fields,
        })
    }
}

fn extract_level(obj: &serde_json::Map<String, Value>) -> LogLevel {
    let level_keys = ["level", "lvl", "severity", "log_level"];

    // json 구조에서 동일한 key를 가진 게 있는지 찾고 매치를 돌림
    for key in &level_keys {
        if let Some(v) = obj.get(*key) {
            let s = match v {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                _ => continue,
            };

            let level = LogLevel::from_str(&s);
            if level != LogLevel::Unknown {
                return level;
            }
        }
    }
    LogLevel::Unknown
}
fn extract_message(obj: &serde_json::Map<String, Value>) -> Option<String> {
    let msg_keys = ["msg", "message", "text", "body"];
    for key in &msg_keys {
        if let Some(Value::String(s)) = obj.get(*key) {
            return Some(s.clone());
        }
    }
    None
}

fn extract_timestamp(obj: &serde_json::Map<String, Value>) -> Option<String> {
    let ts_keys = ["ts", "timestamp", "time", "@timestamp", "datetime"];
    for key in &ts_keys {
        if let Some(v) = obj.get(*key) {
            return Some(v.to_string().trim_matches('"').to_string());
        }
    }
    None
}

#[cfg(test)]

mod tests {
    use std::path::PathBuf;

    use crate::{
        error::AesopError,
        parser::json::{self, ParsedLine},
    };

    use super::*;

    fn dummy_path() -> PathBuf {
        PathBuf::from("test.log")
    }

    #[test]
    fn test_parse_basic_json() {
        let raw = r#"{"level":"ERROR","msg":"token expired","ts":"2026-03-17T10:23:00Z"}"#;
        let parsed = json::ParsedLine::parse(raw, &dummy_path()).unwrap();

        assert_eq!(parsed.level, LogLevel::Error)
    }

    #[test]
    fn test_parse_level_aliases() {
        let raw = r#"{"severity":"CRITICAL","msg":"db down"}"#;
        let parsed = json::ParsedLine::parse(raw, &dummy_path()).unwrap();
        assert_eq!(parsed.level, LogLevel::Fatal);
    }

    #[test]
    fn test_parse_not_json() {
        let raw = "2026-03-17 ERROR something failed";
        let result = json::ParsedLine::parse(raw, &dummy_path());
        assert!(matches!(result, Err(AesopError::NotJsonFormat { .. })));
    }

    #[test]
    fn test_parse_fields_preserved() {
        let raw = r#"{"level":"INFO","msg":"login","user_id":123,"service":"auth"}"#;
        let parsed = ParsedLine::parse(raw, &dummy_path()).unwrap();

        assert!(parsed.fields.contains_key("user_id"));
        assert!(parsed.fields.contains_key("service"));
    }

    #[test]
    fn test_parse_missing_level() {
        let raw = r#"{"msg":"something happened"}"#;
        let parsed = ParsedLine::parse(raw, &dummy_path()).unwrap();
        assert_eq!(parsed.level, LogLevel::Unknown);
    }

    #[test]
    fn test_parse_empty_object() {
        let raw = r#"{}"#;
        let parsed = ParsedLine::parse(raw, &dummy_path()).unwrap();
        assert_eq!(parsed.level, LogLevel::Unknown);
        assert_eq!(parsed.message, None);
    }
}
