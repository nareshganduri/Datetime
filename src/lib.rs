//! A simple Rust library for turning a SystemTime into a date and time
//! (in UTC)
//! and returning a simple time stamp suitable for printing.
use std::ops::{Add, AddAssign};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use cache::Cache;

/// an enum representing each day of the week
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

/// an enum representing each month of the year
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

// cache for lazy computation of date and time
#[derive(Debug)]
struct DtCache {
    year: usize,
    month: Month,
    day: Day,
    date: usize,
    hour: usize,
    minute: usize,
    second: usize,
}

impl DtCache {
    fn from_secs(secs: usize) -> Self {
        let table = (1970..).map(|year| {
            let days_per_year = if is_leap_year(year) { 366 } else { 365 };

            let sec_per_year = days_per_year * 24 * 60 * 60;

            (year, sec_per_year)
        });

        let mut x = secs;
        let mut date_year = 0;

        for (year, sec) in table {
            if x < sec {
                date_year = year;
                break;
            }

            x -= sec;
        }

        let table = [
            (Month::January, 31),
            (
                Month::February,
                if is_leap_year(date_year) { 29 } else { 28 },
            ),
            (Month::March, 31),
            (Month::April, 30),
            (Month::May, 31),
            (Month::June, 30),
            (Month::July, 31),
            (Month::August, 31),
            (Month::September, 30),
            (Month::October, 31),
            (Month::November, 30),
            (Month::December, 31),
        ];

        let mut date_month = Month::January;

        for &(month, days) in table.into_iter() {
            let sec_per_month = days * 24 * 60 * 60;

            if x < sec_per_month {
                date_month = month;
                break;
            }

            x -= sec_per_month;
        }

        let day = x / 24 / 60 / 60;
        x -= day * 24 * 60 * 60;

        let hour = x / 60 / 60;
        x -= hour * 60 * 60;

        let minute = x / 60;
        x -= minute * 60;

        let date_day = get_day(secs);

        DtCache {
            year: date_year,
            month: date_month,
            day: date_day,
            date: day + 1,
            hour: hour,
            minute: minute,
            second: x,
        }
    }
}

/// A struct storing a date and time as measured in UTC
pub struct DateTime {
    secs: usize,
    cache: Cache<DtCache>,
}

fn is_leap_year(year: usize) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

fn get_day(time: usize) -> Day {
    let day = time / 24 / 60 / 60;
    let day = day + 4;
    let day = day % 7;

    match day {
        0 => Day::Sunday,
        1 => Day::Monday,
        2 => Day::Tuesday,
        3 => Day::Wednesday,
        4 => Day::Thursday,
        5 => Day::Friday,
        6 => Day::Saturday,
        _ => unreachable!(),
    }
}

impl DateTime {
    /// return a DateTime corresponding to the current system time
    /// ```
    /// # use datetime::DateTime;
    /// let mut date = DateTime::now();
    /// let time_stamp = date.as_time_stamp();
    ///
    /// println!("The current time is {}", time_stamp);
    /// ```
    pub fn now() -> Self {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        DateTime::from_secs(secs)
    }

    /// returns a DateTime corresponding to a given length of time
    /// (in seconds)
    /// ```
    /// # use datetime::{DateTime, Day, Month};
    /// let mut date = DateTime::from_secs(842282624);
    ///
    /// assert_eq!(date.year(), 1996);
    /// assert_eq!(date.month(), Month::September);
    /// assert_eq!(date.day(), Day::Monday);
    /// assert_eq!(date.date(), 9);
    /// assert_eq!(date.hour(), 15);
    /// assert_eq!(date.minute(), 23);
    /// assert_eq!(date.second(), 44);
    /// ```
    pub fn from_secs(secs: usize) -> Self {
        DateTime {
            secs,
            cache: Cache::new(Box::new(move || DtCache::from_secs(secs))),
        }
    }

    /// returns the DateTime's year
    /// ```
    /// # use datetime::DateTime;
    /// let mut date = DateTime::from_secs(842282624);
    ///
    /// assert_eq!(date.year(), 1996);
    /// ```
    pub fn year(&self) -> usize {
        self.cache.get().year
    }

    /// returns the DateTime's month
    /// ```
    /// # use datetime::DateTime;
    /// # use datetime::Month;
    /// let mut date = DateTime::from_secs(842282624);
    ///
    /// assert_eq!(date.month(), Month::September);
    /// ```
    pub fn month(&self) -> Month {
        self.cache.get().month
    }

    /// returns the DateTime's day
    /// ```
    /// # use datetime::DateTime;
    /// # use datetime::Day;
    /// let mut date = DateTime::from_secs(842282624);
    ///
    /// assert_eq!(date.day(), Day::Monday);
    /// ```
    pub fn day(&self) -> Day {
        self.cache.get().day
    }

    /// returns the DateTime's date
    /// ```
    /// # use datetime::DateTime;
    /// let mut date = DateTime::from_secs(842282624);
    ///
    /// assert_eq!(date.date(), 9);
    /// ```
    pub fn date(&self) -> usize {
        self.cache.get().date
    }

    /// returns the DateTime's hour
    /// ```
    /// # use datetime::DateTime;
    /// let mut date = DateTime::from_secs(842282624);
    ///
    /// assert_eq!(date.hour(), 15);
    /// ```
    pub fn hour(&self) -> usize {
        self.cache.get().hour
    }

    /// returns the DateTime's minute
    /// ```
    /// # use datetime::DateTime;
    /// let mut date = DateTime::from_secs(842282624);
    ///
    /// assert_eq!(date.minute(), 23);
    /// ```
    pub fn minute(&self) -> usize {
        self.cache.get().minute
    }

    /// returns the DateTime's second
    /// ```
    /// # use datetime::DateTime;
    /// let mut date = DateTime::from_secs(842282624);
    ///
    /// assert_eq!(date.second(), 44);
    /// ```
    pub fn second(&self) -> usize {
        self.cache.get().second
    }

    /// returns a String representing the time stamp of a DateTime
    /// ```
    /// # use datetime::DateTime;
    /// let mut date = DateTime::from_secs(842282624);
    /// assert_eq!(date.as_time_stamp(), "Mon Sep 9, 1996  15:23:44 (UTC)");
    /// ```
    pub fn as_time_stamp(&self) -> String {
        let day = match self.day() {
            Day::Sunday => "Sun",
            Day::Monday => "Mon",
            Day::Tuesday => "Tue",
            Day::Wednesday => "Wed",
            Day::Thursday => "Thu",
            Day::Friday => "Fri",
            Day::Saturday => "Sat",
        };

        let month = match self.month() {
            Month::January => "Jan",
            Month::February => "Feb",
            Month::March => "Mar",
            Month::April => "Apr",
            Month::May => "May",
            Month::June => "Jun",
            Month::July => "Jul",
            Month::August => "Aug",
            Month::September => "Sep",
            Month::October => "Oct",
            Month::November => "Nov",
            Month::December => "Dec",
        };

        format!(
            "{} {} {}, {}  {}:{:02}:{:02} (UTC)",
            day,
            month,
            self.date(),
            self.year(),
            self.hour(),
            self.minute(),
            self.second()
        )
    }
}

impl From<SystemTime> for DateTime {
    fn from(time: SystemTime) -> Self {
        let secs = time.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;

        Self::from_secs(secs)
    }
}

impl Add<&DateTime> for DateTime {
    type Output = DateTime;

    fn add(self, other: &DateTime) -> Self {
        let secs = self.secs + other.secs;

        DateTime::from_secs(secs)
    }
}

impl AddAssign<&DateTime> for DateTime {
    fn add_assign(&mut self, other: &DateTime) {
        self.secs += other.secs;

        let secs = self.secs;
        self.cache = Cache::new(Box::new(move || DtCache::from_secs(secs)));
    }
}

#[cfg(test)]
mod tests {
    use super::{DateTime, Day, Month};

    #[test]
    fn test_from_secs() {
        let secs = 842282624;

        let date = DateTime::from_secs(secs);

        assert_eq!(date.year(), 1996);
        assert_eq!(date.month(), Month::September);
        assert_eq!(date.day(), Day::Monday);
        assert_eq!(date.date(), 9);
        assert_eq!(date.hour(), 15);
        assert_eq!(date.minute(), 23);
        assert_eq!(date.second(), 44);
    }

    #[test]
    fn test_add() {
        let date = DateTime::from_secs(123456789);
        let date2 = DateTime::from_secs(234567890);

        let date = date + &date2;

        assert_eq!(date.year(), 1981);
        assert_eq!(date.month(), Month::May);
        assert_eq!(date.day(), Day::Wednesday);
        assert_eq!(date.date(), 6);
        assert_eq!(date.hour(), 19);
        assert_eq!(date.minute(), 17);
        assert_eq!(date.second(), 59);
    }

    #[test]
    fn test_add_assign() {
        let mut date = DateTime::from_secs(123456789);
        let date2 = DateTime::from_secs(234567890);

        date += &date2;

        assert_eq!(date.year(), 1981);
        assert_eq!(date.month(), Month::May);
        assert_eq!(date.day(), Day::Wednesday);
        assert_eq!(date.date(), 6);
        assert_eq!(date.hour(), 19);
        assert_eq!(date.minute(), 17);
        assert_eq!(date.second(), 59);
    }
}
