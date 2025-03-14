#![allow(unused)]
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, Local, LocalResult, NaiveDateTime, TimeZone, Utc};

pub trait FormatDateTime {
    fn to_default(&self) -> String;
    fn to_rfc1123(&self) -> String;
}

const FORMAT_DEFAULT: &str = "%Y-%m-%d %H:%M:%S";
const FORMAT_RFC1123: &str = "%a, %d %b %Y %H:%M:%S %Z";
const FORMAT_RFC3339: &str = "%Y-%m-%dT%H:%M:%S%.6f%:z";

impl<T> FormatDateTime for DateTime<T>
where
    T: TimeZone,
    T::Offset: std::fmt::Display,
{
    fn to_rfc1123(&self) -> String {
        self.format(FORMAT_RFC1123).to_string()
    }

    fn to_default(&self) -> String {
        self.format(FORMAT_DEFAULT).to_string()
    }
}