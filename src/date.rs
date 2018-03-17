use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use chrono::{Date, DateTime, Datelike, Local, TimeZone};
use std::ops::Deref;
use std::fmt;

#[derive(Debug)]
pub struct UntisDate(Date<Local>);

#[derive(Debug)]
pub struct UntisTime(DateTime<Local>);

impl UntisDate {
    pub fn week_begin_from(date: Date<Local>) -> Self {
        let day = date.day();
        let days_from_monday = date.weekday().num_days_from_monday();
        let monday = date.with_day(day - days_from_monday).unwrap();
        UntisDate(monday)
    }

    pub fn week_end_from(date: Date<Local>) -> Self {
        let day = date.day();
        let days_from_monday = date.weekday().num_days_from_monday();
        let days_left_till_friday = 5 - days_from_monday;
        let friday = date.with_day(day + days_left_till_friday).unwrap();
        UntisDate(friday)
    }

    pub fn week_begin() -> Self {
        Self::week_begin_from(Local::today())
    }

    pub fn week_end() -> Self {
        Self::week_end_from(Local::today())
    }
}

impl Deref for UntisDate {
    type Target = Date<Local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for UntisDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(local_to_untis_date_number(**self))
    }
}

impl<'de> Deserialize<'de> for UntisDate {
    fn deserialize<D>(deserializer: D) -> Result<UntisDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(UntisDateVisitor)
    }
}

struct UntisDateVisitor;

impl<'de> Visitor<'de> for UntisDateVisitor {
    type Value = UntisDate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("an integer in the format YEARMONTHDAY like 20180316 for the 16. March 2018")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisDate(local_from_untis_date_number(value as u32)))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisDate(local_from_untis_date_number(value as u32)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisDate(local_from_untis_date_number(value as u32)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisDate(local_from_untis_date_number(value as u32)))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisDate(local_from_untis_date_number(value as u32)))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisDate(local_from_untis_date_number(value as u32)))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisDate(local_from_untis_date_number(value as u32)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisDate(local_from_untis_date_number(value as u32)))
    }
}

fn local_to_untis_date_number(date: Date<Local>) -> u32 {
    let string = format!("{}", date.format("%Y%m%d"));
    let number = string.parse::<u32>().unwrap();
    number
}

fn local_from_untis_date_number(value: u32) -> Date<Local> {
    let string = format!("{}", value);
    let year = string[0..4].parse::<i32>().unwrap();
    let month = string[4..6].parse::<u32>().unwrap();
    let day = string[6..8].parse::<u32>().unwrap();

    Local.ymd(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;

    #[test]
    fn convert_untis_date_forth_and_back() {
        let number = 20180316;
        let date = local_from_untis_date_number(number);
        let new_number = local_to_untis_date_number(date);

        assert_eq!(number, new_number);
    }

    #[test]
    fn untis_date_week_begin_is_monday() {
        let monday = UntisDate::week_begin();
        assert_eq!(monday.0.weekday(), Weekday::Mon);
    }

    #[test]
    fn untis_date_week_end_is_saturday() {
        let monday = UntisDate::week_end();
        assert_eq!(monday.0.weekday(), Weekday::Sat);
    }
}
