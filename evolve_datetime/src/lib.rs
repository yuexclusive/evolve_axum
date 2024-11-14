#![allow(unused)]
use chrono::{DateTime, FixedOffset, Local, LocalResult, NaiveDateTime, TimeZone, Utc};
use evolve_error::{AppError, AppResult};

pub trait FormatDateTime {
    fn to_rfc3339(&self) -> String;
    fn to_default(&self) -> String;
}

const FORMAT_DEFAULT: &str = "%Y-%m-%d %H:%M:%S";
const FORMAT_RFC3339: &str = "%Y-%m-%dT%H:%M:%S%.6f%:z";

pub trait ToDateTime {
    fn default_to_utc(&self) -> AppResult<DateTime<Utc>>;
    fn default_to_local(&self) -> AppResult<DateTime<FixedOffset>>;
    fn rfc3339_to_utc(&self) -> AppResult<DateTime<Utc>>;
    fn rfc3339_to_local(&self) -> AppResult<DateTime<FixedOffset>>;
    fn to_utc(&self, format: &str) -> AppResult<DateTime<Utc>>;
    fn to_local(&self, format: &str) -> AppResult<DateTime<FixedOffset>>;
}

impl FormatDateTime for DateTime<Utc> {
    fn to_rfc3339(&self) -> String {
        DateTime::<Local>::from(self.clone())
            .format(FORMAT_RFC3339)
            .to_string()
    }

    fn to_default(&self) -> String {
        DateTime::<Local>::from(self.clone())
            .format(FORMAT_DEFAULT)
            .to_string()
    }
}

impl<T> ToDateTime for T
where
    T: AsRef<str>,
{
    fn default_to_utc(&self) -> AppResult<DateTime<Utc>> {
        let local_dt = self.default_to_local()?;
        let res = DateTime::<Utc>::from(local_dt);
        Ok(res)
    }

    fn default_to_local(&self) -> AppResult<DateTime<FixedOffset>> {
        let dt = NaiveDateTime::parse_from_str(self.as_ref(), FORMAT_DEFAULT)?;
        match FixedOffset::east_opt(8 * 60 * 60)
            .unwrap()
            .from_local_datetime(&dt)
        {
            LocalResult::Single(v) => Ok(v),
            LocalResult::Ambiguous(x, y) => {
                tracing::warn!("default_to_local ambiguous date {} and {}", x, y);
                Ok(x)
            }
            LocalResult::None => AppError::Internal {
                msg: "default_to_local failed".to_string(),
            }
            .into(),
        }
    }

    fn rfc3339_to_utc(&self) -> AppResult<DateTime<Utc>> {
        let local_dt = self.rfc3339_to_local()?;
        let res = DateTime::<Utc>::from(local_dt);
        Ok(res)
    }

    fn rfc3339_to_local(&self) -> AppResult<DateTime<FixedOffset>> {
        let dt = NaiveDateTime::parse_from_str(self.as_ref(), FORMAT_RFC3339)?;
        match FixedOffset::east_opt(8 * 60 * 60)
            .unwrap()
            .from_local_datetime(&dt)
        {
            LocalResult::Single(v) => Ok(v),
            LocalResult::Ambiguous(x, y) => {
                tracing::warn!("rfc3339_to_local ambiguous date {} and {}", x, y);
                Ok(x)
            }
            LocalResult::None => AppError::Internal {
                msg: "rfc3339_to_local failed".to_string(),
            }
            .into(),
        }
    }

    fn to_utc(&self, format: &str) -> AppResult<DateTime<Utc>> {
        let local_dt = self.to_local(format)?;
        let res = DateTime::<Utc>::from(local_dt);
        Ok(res)
    }

    fn to_local(&self, format: &str) -> AppResult<DateTime<FixedOffset>> {
        let dt = NaiveDateTime::parse_from_str(self.as_ref(), format)?;
        match FixedOffset::east_opt(8 * 60 * 60)
            .unwrap()
            .from_local_datetime(&dt)
        {
            LocalResult::Single(v) => Ok(v),
            LocalResult::Ambiguous(x, y) => {
                tracing::warn!("to_local ambiguous date {} and {}", x, y);
                Ok(x)
            }
            LocalResult::None => AppError::Internal {
                msg: "to_local failed".to_string(),
            }
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_to_local() {
        let str = "2023-09-01 12:00:00";
        let res = str.default_to_local().unwrap();
        assert_eq!(format!("{}", res), "2023-09-01 12:00:00 +08:00");
    }

    #[test]
    fn test_default_to_utc() {
        let str = "2023-09-01 12:00:00";
        let res = str.default_to_utc().unwrap();
        assert_eq!(format!("{}", res), "2023-09-01 04:00:00 UTC");
    }

    #[test]
    fn test_rfc3339_to_local() {
        let str = "2023-09-01T12:00:00+00:00";
        let res = str.rfc3339_to_local().unwrap();
        assert_eq!(format!("{}", res), "2023-09-01 12:00:00 +08:00");
    }

    #[test]
    fn test_rfc3339_to_utc() {
        let str = "2023-09-01T12:00:00+00:00";
        let res = str.rfc3339_to_utc().unwrap();
        assert_eq!(format!("{}", res), "2023-09-01 04:00:00 UTC");
    }

    #[test]
    fn test_to_local() {
        let str = "2023-09-01AA12:00:00";
        let res = str.to_local("%Y-%m-%dAA%H:%M:%S").unwrap();
        assert_eq!(format!("{}", res), "2023-09-01 12:00:00 +08:00");
    }

    #[test]
    fn test_to_utc() {
        let str = "2023-09-01AA12:00:00";
        let res = str.to_utc("%Y-%m-%dAA%H:%M:%S").unwrap();
        assert_eq!(format!("{}", res), "2023-09-01 04:00:00 UTC");
    }
}
