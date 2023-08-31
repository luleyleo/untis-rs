use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use serde::de::Visitor;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Date(NaiveDate);

impl Date {
    pub fn week_begin_from(date: NaiveDate) -> Self {
        let days_from_monday = date.weekday().num_days_from_monday();
        let monday = date - Duration::days(days_from_monday as i64);
        Date(monday)
    }

    pub fn week_end_from(date: NaiveDate) -> Self {
        let days_from_monday = date.weekday().num_days_from_monday() as i64;
        let days_left_till_saturday = 5 - days_from_monday;
        let saturday = date + Duration::days(days_left_till_saturday);
        Date(saturday)
    }

    pub fn week_begin() -> Self {
        Self::week_begin_from(Local::now().date_naive())
    }

    pub fn week_end() -> Self {
        Self::week_end_from(Local::now().date_naive())
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

    fn visit_i8<E: serde::de::Error>(self, value: i8) -> Result<Self::Value, E> {
        Ok(Date(chrono_from_untis_date(value as u32)))
    }

    fn visit_i16<E: serde::de::Error>(self, value: i16) -> Result<Self::Value, E> {
        Ok(Date(chrono_from_untis_date(value as u32)))
    }

    fn visit_i32<E: serde::de::Error>(self, value: i32) -> Result<Self::Value, E> {
        Ok(Date(chrono_from_untis_date(value as u32)))
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Date(chrono_from_untis_date(value as u32)))
    }

    fn visit_u8<E: serde::de::Error>(self, value: u8) -> Result<Self::Value, E> {
        Ok(Date(chrono_from_untis_date(value as u32)))
    }

    fn visit_u16<E: serde::de::Error>(self, value: u16) -> Result<Self::Value, E> {
        Ok(Date(chrono_from_untis_date(value as u32)))
    }

    fn visit_u32<E: serde::de::Error>(self, value: u32) -> Result<Self::Value, E> {
        Ok(Date(chrono_from_untis_date(value as u32)))
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Time(NaiveTime);

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

    fn visit_i8<E: serde::de::Error>(self, value: i8) -> Result<Self::Value, E> {
        Ok(Time(chrono_from_untis_time(value as u32)))
    }

    fn visit_i16<E: serde::de::Error>(self, value: i16) -> Result<Self::Value, E> {
        Ok(Time(chrono_from_untis_time(value as u32)))
    }

    fn visit_i32<E: serde::de::Error>(self, value: i32) -> Result<Self::Value, E> {
        Ok(Time(chrono_from_untis_time(value as u32)))
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Time(chrono_from_untis_time(value as u32)))
    }

    fn visit_u8<E: serde::de::Error>(self, value: u8) -> Result<Self::Value, E> {
        Ok(Time(chrono_from_untis_time(value as u32)))
    }

    fn visit_u16<E: serde::de::Error>(self, value: u16) -> Result<Self::Value, E> {
        Ok(Time(chrono_from_untis_time(value as u32)))
    }

    fn visit_u32<E: serde::de::Error>(self, value: u32) -> Result<Self::Value, E> {
        Ok(Time(chrono_from_untis_time(value as u32)))
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
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
        let monday = Date::week_begin();
        assert_eq!(monday.0.weekday(), Weekday::Mon);
    }

    #[test]
    fn untis_date_week_end_is_saturday() {
        let saturday = Date::week_end();
        assert_eq!(saturday.0.weekday(), Weekday::Sat);
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