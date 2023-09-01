use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use serde::de::Visitor;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

/// Wrapper around chrono::NaiveDate for working with Untis more easily.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Date(pub NaiveDate);

impl Date {
    /// Returns the current date.
    pub fn today() -> Self {
        Date(Local::now().date_naive())
    }

    /// Returns the last start of the week (monday).
    pub fn current_week_begin() -> Self {
        Self::today().relative_week_begin()
    }

    /// Returns the next end of the week (saturday).
    pub fn current_week_end() -> Self {
        Self::today().relative_week_end()
    }

    /// Returns the last start of the week before this date.
    pub fn relative_week_begin(&self) -> Self {
        let days_from_monday = self.weekday().num_days_from_monday();
        let monday = self.0 - Duration::days(days_from_monday as i64);
        Date(monday)
    }

    /// Returns the next end of the week (saturday) after this date.
    pub fn relative_week_end(&self) -> Self {
        let days_from_monday = self.weekday().num_days_from_monday() as i64;
        let days_left_till_saturday = 5 - days_from_monday;
        let saturday = self.0 + Duration::days(days_left_till_saturday);
        Date(saturday)
    }

    /// Returns the inner `NaiveDate`.
    pub fn to_chrono(&self) -> NaiveDate {
        self.0
    }
}

impl Deref for Date {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(chrono_to_untis_date(**self))
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
        deserializer.deserialize_u32(DateVisitor)
    }
}

struct DateVisitor;

impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("an integer in the format YEARMONTHDAY like 20180316 for the 16. March 2018")
    }

    fn visit_u32<E: serde::de::Error>(self, value: u32) -> Result<Self::Value, E> {
        Ok(Date(chrono_from_untis_date(value as u32)))
    }
}

fn chrono_to_untis_date(date: NaiveDate) -> u32 {
    let string = format!("{}", date.format("%Y%m%d"));
    let number = string.parse::<u32>().unwrap();
    number
}

fn chrono_from_untis_date(value: u32) -> NaiveDate {
    let string = format!("{}", value);
    let year = string[0..4].parse::<i32>().unwrap();
    let month = string[4..6].parse::<u32>().unwrap();
    let day = string[6..8].parse::<u32>().unwrap();

    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

/// Wrapper around chrono::NaiveDate for working with Untis more easily.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Time(pub NaiveTime);

impl Time {
    pub fn to_chrono(&self) -> NaiveTime {
        self.0
    }
}

impl Deref for Time {
    type Target = NaiveTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Time {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(chrono_to_untis_time(**self))
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Time, D::Error> {
        deserializer.deserialize_u32(TimeVisitor)
    }
}

struct TimeVisitor;

impl<'de> Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer in the format HOURMINUTE like 1912 or 712 for 19:12")
    }

    fn visit_u32<E: serde::de::Error>(self, value: u32) -> Result<Self::Value, E> {
        Ok(Time(chrono_from_untis_time(value as u32)))
    }
}

fn chrono_to_untis_time(time: NaiveTime) -> u32 {
    let string = format!("{}", time.format("%k%M"));
    let number = string.trim().parse::<u32>().unwrap();
    number
}

fn chrono_from_untis_time(value: u32) -> NaiveTime {
    let string = format!("{}", value);
    let hour_len = if string.len() == 3 { 1 } else { 2 };
    let hours = string[0..hour_len].parse::<u32>().unwrap();
    let mins = string[hour_len..(hour_len + 2)].parse::<u32>().unwrap();

    NaiveTime::from_hms_opt(hours, mins, 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;

    #[test]
    fn convert_untis_date_forth_and_back() {
        let number = 20180316;
        let date = chrono_from_untis_date(number);
        let new_number = chrono_to_untis_date(date);

        assert_eq!(number, new_number);
    }

    #[test]
    fn untis_date_week_begin_is_monday() {
        let monday = Date::current_week_begin();
        assert_eq!(monday.0.weekday(), Weekday::Mon);
    }

    #[test]
    fn untis_date_week_begin_is_last_monday() {
        let date = Date(NaiveDate::from_ymd_opt(2023, 09, 01).unwrap());
        let monday = Date(NaiveDate::from_ymd_opt(2023, 08, 28).unwrap());
        assert_eq!(monday, date.relative_week_begin());
    }

    #[test]
    fn untis_date_week_end_is_saturday() {
        let saturday = Date::current_week_end();
        assert_eq!(saturday.0.weekday(), Weekday::Sat);
    }

    #[test]
    fn untis_date_week_begin_is_next_saturday() {
        let date = Date(NaiveDate::from_ymd_opt(2023, 09, 01).unwrap());
        let monday = Date(NaiveDate::from_ymd_opt(2023, 09, 02).unwrap());
        assert_eq!(monday, date.relative_week_end());
    }

    #[test]
    fn convert_untis_time_forth_and_back() {
        let number = 830;
        let time = chrono_from_untis_time(number);
        println!("Time: {}", time);
        let new_number = chrono_to_untis_time(time);
        println!("Num:  {}", new_number);

        assert_eq!(number, new_number);
    }
}
