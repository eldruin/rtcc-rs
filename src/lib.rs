//! Data structures and traits to be implemented by
//! real-time clock / calendar devices.

#![deny(unsafe_code, missing_docs)]
#![no_std]

/// Date and time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime {
    /// Year (e.g. 2000)
    pub year    : u16,
    /// Month [1-12]
    pub month   : u8,
    /// Day [1-31]
    pub day     : u8,
    /// Weekday [1-7]
    pub weekday : u8,
    /// Hour in 24h/12h format
    pub hour    : Hours,
    /// Minute [0-59]
    pub minute  : u8,
    /// Second [0-59]
    pub second  : u8,
}

/// Hours in either 12-hour (AM/PM) or 24-hour format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hours {
    /// AM [1-12]
    AM(u8),
    /// PM [1-12]
    PM(u8),
    /// 24H format [0-23]
    H24(u8),
}

/// Real-Time Clock / Calendar
pub trait Rtcc {
    /// Error type
    type Error;

    /// Read the seconds.
    fn get_seconds(&mut self) -> Result<u8, Self::Error>;

    /// Read the minutes.
    fn get_minutes(&mut self) -> Result<u8, Self::Error>;

    /// Read the hours.
    fn get_hours(&mut self) -> Result<Hours, Self::Error>;

    /// Read the day of the week [1-7].
    fn get_weekday(&mut self) -> Result<u8, Self::Error>;

    /// Read the day of the month [1-31].
    fn get_day(&mut self) -> Result<u8, Self::Error>;

    /// Read the month [1-12].
    fn get_month(&mut self) -> Result<u8, Self::Error>;

    /// Read the year (e.g. 2000).
    fn get_year(&mut self) -> Result<u16, Self::Error>;

    /// Read the date and time.
    fn get_datetime(&mut self) -> Result<DateTime, Self::Error>;

    /// Set the seconds [0-59].
    fn set_seconds(&mut self, seconds: u8) -> Result<(), Self::Error>;

    /// Set the minutes [0-59].
    fn set_minutes(&mut self, minutes: u8) -> Result<(), Self::Error>;

    /// Set the hours.
    ///
    /// Changes the operating mode to 12h/24h depending on the parameter.
    fn set_hours(&mut self, hours: Hours) -> Result<(), Self::Error>;

    /// Set the day of week [1-7].
    fn set_weekday(&mut self, weekday: u8) -> Result<(), Self::Error>;

    /// Set the day of month [1-31].
    fn set_day(&mut self, day: u8) -> Result<(), Self::Error>;

    /// Set the month [1-12].
    fn set_month(&mut self, month: u8) -> Result<(), Self::Error>;

    /// Set the year. (e.g. 2000)
    fn set_year(&mut self, year: u16) -> Result<(), Self::Error>;

    /// Set the date and time.
    fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), Self::Error>;
}
