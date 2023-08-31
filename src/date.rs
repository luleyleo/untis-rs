use chrono::{Datelike, Duration, Local, NaiveDate};
use serde::de::Visitor;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UntisDate(NaiveDate);

impl UntisDate {
    pub fn week_begin_from(date: NaiveDate) -> Self {
        let days_from_monday = date.weekday().num_days_from_monday();
        let monday = date - Duration::days(days_from_monday as i64);
        UntisDate(monday)
    }

    pub fn week_end_from(date: NaiveDate) -> Self {
        let days_from_monday = date.weekday().num_days_from_monday() as i64;
        let days_left_till_saturday = 5 - days_from_monday;
        let saturday = date + Duration::days(days_left_till_saturday);
        UntisDate(saturday)
    }

    pub fn week_begin() -> Self {
        Self::week_begin_from(Local::now().date_naive())
    }

    pub fn week_end() -> Self {
        Self::week_end_from(Local::now().date_naive())
    }
}

impl Deref for UntisDate {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for UntisDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(chrono_to_untis_date(**self))
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
    where
        E: serde::de::Error,
    {
        Ok(UntisDate(chrono_from_untis_date(value as u32)))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UntisDate(chrono_from_untis_date(value as u32)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UntisDate(chrono_from_untis_date(value as u32)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UntisDate(chrono_from_untis_date(value as u32)))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UntisDate(chrono_from_untis_date(value as u32)))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UntisDate(chrono_from_untis_date(value as u32)))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UntisDate(chrono_from_untis_date(value as u32)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(UntisDate(chrono_from_untis_date(value as u32)))
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
        let monday = UntisDate::week_begin();
        assert_eq!(monday.0.weekday(), Weekday::Mon);
    }

    #[test]
    fn untis_date_week_end_is_saturday() {
        let saturday = UntisDate::week_end();
        assert_eq!(saturday.0.weekday(), Weekday::Sat);
    }
}
