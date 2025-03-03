//! Data structures and traits to be implemented by real-time clock / calendar devices.
//!
//! Prefer to use only the methods from the `DateTimeAccess` rather than the individual
//! methods from the `Rtcc` trait to avoid situations where the passing of time
//! makes the results of the method calls inconsistent if you combine the results
//! of several methods.
//!
//! For example, this can happen at certain timepoints:
//! 1. The time is `01:59:59`
//! 2. A call to `hours()` returns 1.
//! 3. The time is increased to `02:00:00`.
//! 4. A call to `minutes()` returns 0.
//! 5. A call to `seconds()` returns 0.
//! 6. Your system thinks it is `01:00:00`.
//!
//! The same applies to the date as well, as well as when calling setter methods.
//!
//! # Features
//!
//! The optional `defmt-03` feature implements [defmt::Format] on some types.

#![deny(unsafe_code, missing_docs)]
#![no_std]

pub use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

/// Hours in either 12-hour (AM/PM) or 24-hour format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Hours {
    /// AM [1-12]
    AM(u8),
    /// PM [1-12]
    PM(u8),
    /// 24H format [0-23]
    H24(u8),
}

/// Real-Time Clock / Calendar DateTimeAccess trait to read/write a complete
/// date/time.
///
/// Prefer to use only these methods rather than the individual methods from the
/// `Rtcc` trait to avoid situations where the passing of time makes the results
/// of the method calls inconsistent if you combine the results of several methods.
///
/// For example, this can happen at certain timepoints:
/// 1. The time is `01:59:59`
/// 2. A call to `hours()` returns 1.
/// 3. The time is increased to `02:00:00`.
/// 4. A call to `minutes()` returns 0.
/// 5. A call to `seconds()` returns 0.
/// 6. Your system thinks it is `01:00:00`.
///
/// The same applies to the date as well, as well as when calling setter methods.
pub trait DateTimeAccess {
    /// Error type
    type Error;

    /// Read the date and time.
    fn datetime(&mut self) -> Result<NaiveDateTime, Self::Error>;

    /// Set the date and time.
    ///
    /// This will set the hour operating mode to 24h and the weekday to the
    /// day number starting from Sunday = 1.
    fn set_datetime(&mut self, datetime: &NaiveDateTime) -> Result<(), Self::Error>;
}

/// Real-Time Clock / Calendar trait
///
/// If you want to combine calls to these methods, prefer to use only
/// the `DateTimeAccess` trait to avoid situations where the passing of time makes the results
/// of the method calls inconsistent.
///
/// For example, this can happen at certain timepoints:
/// 1. The time is `01:59:59`
/// 2. A call to `hours()` returns 1.
/// 3. The time is increased to `02:00:00`.
/// 4. A call to `minutes()` returns 0.
/// 5. A call to `seconds()` returns 0.
/// 6. Your system thinks it is `01:00:00`.
///
/// The same applies to the date, as well as when calling setter methods.
pub trait Rtcc: DateTimeAccess {
    /// Read the seconds.
    fn seconds(&mut self) -> Result<u8, Self::Error>;

    /// Read the minutes.
    fn minutes(&mut self) -> Result<u8, Self::Error>;

    /// Read the hours.
    fn hours(&mut self) -> Result<Hours, Self::Error>;

    /// Read the time.
    fn time(&mut self) -> Result<NaiveTime, Self::Error>;

    /// Read the day of the week [1-7].
    fn weekday(&mut self) -> Result<u8, Self::Error>;

    /// Read the day of the month [1-31].
    fn day(&mut self) -> Result<u8, Self::Error>;

    /// Read the month [1-12].
    fn month(&mut self) -> Result<u8, Self::Error>;

    /// Read the year (e.g. 2000).
    fn year(&mut self) -> Result<u16, Self::Error>;

    /// Read the date.
    fn date(&mut self) -> Result<NaiveDate, Self::Error>;

    /// Set the seconds [0-59].
    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error>;

    /// Set the minutes [0-59].
    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error>;

    /// Set the hours.
    ///
    /// Changes the operating mode to 12h/24h depending on the parameter.
    fn set_hours(&mut self, hours: Hours) -> Result<(), Self::Error>;

    /// Set the time.
    fn set_time(&mut self, time: &NaiveTime) -> Result<(), Self::Error>;

    /// Set the day of week [1-7].
    fn set_weekday(&mut self, weekday: u8) -> Result<(), Self::Error>;

    /// Set the day of month [1-31].
    fn set_day(&mut self, day: u8) -> Result<(), Self::Error>;

    /// Set the month [1-12].
    fn set_month(&mut self, month: u8) -> Result<(), Self::Error>;

    /// Set the year. (e.g. 2000)
    fn set_year(&mut self, year: u16) -> Result<(), Self::Error>;

    /// Set the date.
    fn set_date(&mut self, date: &NaiveDate) -> Result<(), Self::Error>;
}
