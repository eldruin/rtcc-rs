use core::fmt::Display;

/// Naive date and time.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NaiveDateTime {
    date: NaiveDate,
    time: NaiveTime,
}

/// Naive date.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NaiveDate {
    year: u16,
    month: u8,
    day: u8,
}

impl NaiveDate {
    /// Create a new NaiveDate from year, month and day.
    ///
    /// Returns None if the year, month or day is invalid.
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        if !(2000..=2173).contains(&year) || !(1..=12).contains(&month) || !(1..=31).contains(&day)
        {
            return None;
        }
        if month == 2 && day > 29 {
            return None;
        }
        if month == 4 || month == 6 || month == 9 || month == 11 && day > 30 {
            return None;
        }
        Some(NaiveDate {
            year: year as u16,
            month: month as u8,
            day: day as u8,
        })
    }

    /// Create a new NaiveDateTime from a NaiveDate and a NaiveTime.
    ///
    /// Returns None if the hours, minutes or seconds is invalid.
    pub fn and_hms_opt(self, hours: u32, minutes: u32, seconds: u32) -> Option<NaiveDateTime> {
        Some(NaiveDateTime {
            date: self,
            time: NaiveTime::from_hms_opt(hours, minutes, seconds)?,
        })
    }
}

impl Display for NaiveDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl NaiveDateTime {
    /// Read the year.
    pub fn year(self) -> i32 {
        self.date.year()
    }
    /// Read the month.
    pub fn month(self) -> u32 {
        self.date.month()
    }
    /// Read the day.
    pub fn day(self) -> u32 {
        self.date.day()
    }
    /// Read the hour.
    pub fn hour(self) -> u32 {
        self.time.hour()
    }
    /// Read the minute.
    pub fn minute(self) -> u32 {
        self.time.minute()
    }
    /// Read the second.
    pub fn second(self) -> u32 {
        self.time.second()
    }
    /// Read the weekday.
    pub fn weekday(self) -> Weekday {
        self.date.weekday()
    }
}

/// Naive time.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NaiveTime {
    hour: u8,
    minute: u8,
    second: u8,
}

impl NaiveTime {
    /// Create a new NaiveTime from hours, minutes and seconds.
    ///
    /// Returns None if the hours, minutes or seconds is invalid.
    pub fn from_hms_opt(hours: u32, minutes: u32, seconds: u32) -> Option<NaiveTime> {
        if hours > 23 || minutes > 59 || seconds > 60 {
            return None;
        }
        Some(NaiveTime {
            hour: hours as u8,
            minute: minutes as u8,
            second: seconds as u8,
        })
    }
}

impl Display for NaiveTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

/// Datelike trait.
pub trait Datelike {
    /// Read the year.
    fn year(&self) -> i32;
    /// Read the month.
    fn month(&self) -> u32;
    /// Read the day.
    fn day(&self) -> u32;
    /// Read the weekday.
    fn weekday(&self) -> Weekday;
}

impl Datelike for NaiveDate {
    fn year(&self) -> i32 {
        i32::from(self.year)
    }
    fn month(&self) -> u32 {
        u32::from(self.month)
    }
    fn day(&self) -> u32 {
        u32::from(self.day)
    }

    /// Returns the weekday of the date.
    ///
    /// This is guaranteed to work for all valid dates within the years 2000 to 2173.
    ///
    /// Invalid dates will return arbitrary results or may panic.
    fn weekday(&self) -> Weekday {
        let year = self.year.saturating_sub(2000) as u8;

        // good enough before 2200
        let is_leap_year = || year.is_multiple_of(4) && year != 100;

        // number of week-day shifts for each month of a non-leap year
        let months = [
            0_u8, // enables 1-based indexing of months
            0,    // January
            31 % 7,
            59 % 7,
            90 % 7,
            120 % 7,
            151 % 7,
            181 % 7,
            212 % 7,
            243 % 7,
            (273 % 7) as u8,
            (304 % 7) as u8,
            (334 % 7) as u8, // December
        ];

        // number of week-day shifts for the year
        // one extra-shift for each leap year (with a correction for the skipped year 2100)
        let year_ordinal = year + year.div_ceil(4) - u8::from(year > 100);

        // insert Feb. 29 for leap years
        let leap_ordinal = u8::from(self.month > 2 && is_leap_year());

        // number of week-day shifts for the month
        let month_ordinal = *months.get(usize::from(self.month)).unwrap_or(&0);

        // number of week-day shifts for the day
        let day_ordinal = self.day;

        // total number of week-day shifts
        let ordinal_date = year_ordinal + leap_ordinal + month_ordinal + day_ordinal;

        // this is the only real division
        let weekday = (ordinal_date + 4) % 7;
        match weekday {
            0 => Weekday::Mon,
            1 => Weekday::Tue,
            2 => Weekday::Wed,
            3 => Weekday::Thu,
            4 => Weekday::Fri,
            5 => Weekday::Sat,
            6 => Weekday::Sun,
            _ => unreachable!(),
        }
    }
}

impl Timelike for NaiveTime {
    fn hour(&self) -> u32 {
        u32::from(self.hour)
    }
    fn minute(&self) -> u32 {
        u32::from(self.minute)
    }
    fn second(&self) -> u32 {
        u32::from(self.second)
    }
}

/// Timelike trait.
pub trait Timelike {
    /// Read the hour.
    fn hour(&self) -> u32;
    /// Read the minute.
    fn minute(&self) -> u32;
    /// Read the second.
    fn second(&self) -> u32;
}

impl Timelike for NaiveDateTime {
    fn hour(&self) -> u32 {
        self.time.hour()
    }
    fn minute(&self) -> u32 {
        self.time.minute()
    }
    fn second(&self) -> u32 {
        self.time.second()
    }
}

impl Datelike for NaiveDateTime {
    fn year(&self) -> i32 {
        self.date.year()
    }
    fn month(&self) -> u32 {
        self.date.month()
    }
    fn day(&self) -> u32 {
        self.date.day()
    }
    fn weekday(&self) -> Weekday {
        self.date.weekday()
    }
}

/// Weekday.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Weekday {
    /// Monday.
    Mon = 0,
    /// Tuesday.
    Tue = 1,
    /// Wednesday.
    Wed = 2,
    /// Thursday.
    Thu = 3,
    /// Friday.
    Fri = 4,
    /// Saturday.
    Sat = 5,
    /// Sunday.
    Sun = 6,
}

impl Weekday {
    /// Returns a day-of-week number starting from Sunday = 1.
    pub fn number_from_sunday(self) -> u32 {
        // avoid modulo operation for embedded targets
        if self == Self::Sun {
            1
        } else {
            u32::from(self as u8) + 2
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Datelike as _;

    use crate::Datelike;

    #[test]
    fn test_weekday() {
        for year in 2000..2173 {
            'next_month: for month in 1..12 {
                for day in 1..31 {
                    let Some(chrono_date) = chrono::NaiveDate::from_ymd_opt(year, month, day)
                    else {
                        continue 'next_month;
                    };

                    let rtcc_date = super::NaiveDate {
                        year: year as u16,
                        month: month as u8,
                        day: day as u8,
                    };

                    assert_eq!(
                        chrono_date.weekday() as u8,
                        rtcc_date.weekday() as u8,
                        "{year:04}-{month:02}-{day:02}",
                    );
                }
            }
        }
    }
}
